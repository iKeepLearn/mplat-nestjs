import { HttpService } from '@nestjs/axios';
import { Injectable, OnModuleInit } from '@nestjs/common';
import * as crypto from 'crypto';
import { PrismaService } from 'src/prisma/prisma.service';
import { getDateTime, isAfterTime, timeAdd } from 'src/utils';
const ALGORITHM = 'aes-256-cbc'; // 使用的加密算法
const MSG_LENGTH_SIZE = 4; // 存放消息体尺寸的空间大小。单位：字节
const RANDOM_BYTES_SIZE = 16; // 随机数据的大小。单位：字节
const BLOCK_SIZE = 32; // 分块尺寸。单位：字节

type AppInfo = {
  appId: string;
  key: Buffer;
  token: string;
  iv: Buffer;
  secret: string;
};

@Injectable()
export class WxService implements OnModuleInit {
  private appInfo = {} as AppInfo;
  constructor(private prisma: PrismaService, private axios: HttpService) {}

  async onModuleInit() {
    const result = await this.prisma.commKv.findMany({
      where: { key: { in: ['appid', 'encodingAESKey', 'token', 'secret'] } },
      select: { key: true, value: true },
    });
    const appId = result.find((item: any) => item.key == 'appid');
    this.appInfo.appId = appId.value;
    const encodingAESKey = result.find(
      (item: any) => item.key == 'encodingAESKey',
    ).value;
    const token = result.find((item: any) => item.key == 'token').value;
    const secret = result.find((item: any) => item.key == 'secret').value;
    const key = Buffer.from(encodingAESKey + '=', 'base64');
    this.appInfo.token = token;
    this.appInfo.secret = secret;
    this.appInfo.key = Buffer.from(encodingAESKey + '=', 'base64');
    this.appInfo.iv = key.slice(0, 16);
  }

  /**
   * 加密消息
   * @param {string} msg 待加密的消息体
   */
  encode(msg) {
    const { appId, key, iv } = this.appInfo;
    const randomBytes = crypto.randomBytes(RANDOM_BYTES_SIZE); // 生成指定大小的随机数据

    let msgLenBuf = Buffer.alloc(MSG_LENGTH_SIZE); // 申请指定大小的空间，存放消息体的大小
    const offset = 0; // 写入的偏移值
    msgLenBuf.writeUInt32BE(Buffer.byteLength(msg), offset); // 按大端序（网络字节序）写入消息体的大小

    const msgBuf = Buffer.from(msg); // 将消息体转成 buffer
    const appIdBuf = Buffer.from(appId); // 将 APPID 转成 buffer

    let totalBuf = Buffer.concat([randomBytes, msgLenBuf, msgBuf, appIdBuf]); // 将16字节的随机数据、4字节的消息体大小、若干字节的消息体、若干字节的APPID拼接起来

    let cipher = crypto.createCipheriv(ALGORITHM, key, iv); // 创建加密器实例
    cipher.setAutoPadding(false); // 禁用默认的数据填充方式
    totalBuf = this.PKCS7Encode(totalBuf); // 使用自定义的数据填充方式
    const encryptdBuf = Buffer.concat([
      cipher.update(totalBuf),
      cipher.final(),
    ]); // 加密后的数据

    return encryptdBuf.toString('base64'); // 返回加密数据的 base64 编码结果
  }

  /**
   * 解密消息
   * @param {string} encryptdMsg 待解密的消息体
   */
  decode(encryptdMsg) {
    const { key, iv } = this.appInfo;
    const encryptedMsgBuf = Buffer.from(encryptdMsg, 'base64'); // 将 base64 编码的数据转成 buffer
    let decipher = crypto.createDecipheriv(ALGORITHM, key, iv); // 创建解密器实例
    decipher.setAutoPadding(false); // 禁用默认的数据填充方式
    let decryptdBuf = Buffer.concat([
      decipher.update(encryptedMsgBuf),
      decipher.final(),
    ]); // 解密后的数据

    decryptdBuf = this.PKCS7Decode(decryptdBuf); // 去除填充的数据

    const msgSize = decryptdBuf.readUInt32BE(RANDOM_BYTES_SIZE); // 根据指定偏移值，从 buffer 中读取消息体的大小，单位：字节
    const msgBufStartPos = RANDOM_BYTES_SIZE + MSG_LENGTH_SIZE; // 消息体的起始位置
    const msgBufEndPos = msgBufStartPos + msgSize; // 消息体的结束位置

    const msgBuf = decryptdBuf.slice(msgBufStartPos, msgBufEndPos); // 从 buffer 中提取消息体

    return msgBuf.toString(); // 将消息体转成字符串，并返回数据
  }

  /**
   * 生成签名
   * @param {Object} params 待签名的参数
   */
  genSign(params) {
    const { token } = this.appInfo;
    const { timestamp, nonce, encrypt } = params;
    const rawStr = [token, timestamp, nonce, encrypt].sort().join(''); // 原始字符串
    const signature = crypto.createHash('sha1').update(rawStr).digest('hex'); // 计算签名
    return signature;
  }

  /**
   * 按 PKCS#7 的方式从填充过的数据中提取原数据
   * @param {Buffer} buf 待处理的数据
   */
  PKCS7Decode(buf) {
    const padSize = buf[buf.length - 1]; // 最后1字节记录着填充的数据大小
    return buf.slice(0, buf.length - padSize); // 提取原数据
  }

  /**
   * 按 PKCS#7 的方式填充数据结尾
   * @param {Buffer} buf 待填充的数据
   */
  PKCS7Encode(buf) {
    const padSize = BLOCK_SIZE - (buf.length % BLOCK_SIZE); // 计算填充的大小。
    const fillByte = padSize; // 填充的字节数据为填充的大小
    const padBuf = Buffer.alloc(padSize, fillByte); // 分配指定大小的空间，并填充数据
    return Buffer.concat([buf, padBuf]); // 拼接原数据和填充的数据
  }

  async getComponentToken() {
    const appid = this.appInfo.appId;
    const now = getDateTime(Date.now());
    const token = await this.prisma.wxToken.findFirst({
      where: { appid, type: 'component_access_token' },
      select: { token: true, expire_time: true },
    });
    //     const expire_time = timeAdd(now, 7200, 's');
    //     console.log({now,expire_time})
    // return
    if (token && isAfterTime(token.expire_time, now)) {
      return token.token;
    } else {
      const appSecret = this.appInfo.secret;
      const ticket = await this.prisma.commKv.findFirst({
        where: { key: 'ticket' },
        select: { value: true },
      });

      const result = await this.axios.axiosRef.post(
        'https://api.weixin.qq.com/cgi-bin/component/api_component_token',
        {
          component_appid: appid,
          component_appsecret: appSecret,
          component_verify_ticket: ticket.value,
        },
        { proxy: false },
      );
      const { component_access_token, expires_in } = result.data;
      const expire_time = timeAdd(now, expires_in, 's');

      const addToken = await this.prisma.wxToken.upsert({
        where: { appid_type: { appid, type: 'component_access_token' } },
        update: { token: component_access_token, expire_time },
        create: {
          appid,
          type: 'component_access_token',
          token: component_access_token,
          expire_time,
        },
      });

      return component_access_token;
    }
  }

  async getPreauthCode() {
    const token = await this.getComponentToken();
    const appid = this.appInfo.appId;
    const result = await this.axios.axiosRef.post(
      `https://api.weixin.qq.com/cgi-bin/component/api_create_preauthcode?component_access_token=${token}`,
      { component_appid: appid },
      { proxy: false },
    );
    // console.log({ token, data: result.data });
    return result.data;
  }

  async getAuthorizerList(count = 100, offset = 0) {
    const token = await this.getComponentToken();
    const appid = this.appInfo.appId;
    const result = await this.axios.axiosRef.post(
      `https://api.weixin.qq.com/cgi-bin/component/api_get_authorizer_list?access_token=${token}`,
      { component_appid: appid, count, offset },
      { proxy: false },
    );
    // console.log({ token, data: result.data });
    return result.data;
  }

  async getAuthorizerInfo(targetId) {
    const token = await this.getComponentToken();
    const appid = this.appInfo.appId;
    const result = await this.axios.axiosRef.post(
      `https://api.weixin.qq.com/cgi-bin/component/api_get_authorizer_info?access_token=${token}`,
      { component_appid: appid, authorizer_appid: targetId },
      { proxy: false },
    );
    // console.log({ data: result.data });
    return result.data;
  }
}
