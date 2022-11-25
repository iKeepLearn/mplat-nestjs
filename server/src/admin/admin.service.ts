import { Injectable } from '@nestjs/common';
import { ConfigService } from '@nestjs/config';
import * as bcrypt from 'bcrypt';
import { CustomException } from 'src/exceptions/custom.exception';
import { PrismaService } from 'src/prisma/prisma.service';
import { getDateTime, timeAdd } from 'src/utils';
import { WxService } from 'src/wx/wx.service';
import { ChangePwdDto } from './dto/change-pwd.dto';

@Injectable()
export class AdminService {
  constructor(
    private prisma: PrismaService,
    private config: ConfigService,
    private wx: WxService,
  ) {}

  async hashPassword(password: string) {
    const saltOrRounds = parseInt(this.config.get('BCRYPT_SALT_LENGTH'));
    return await bcrypt.hash(password, saltOrRounds);
  }

  async changePassword(detail: ChangePwdDto, username: string) {
    const encryptPassword = await this.hashPassword(detail.password);
    const result = await this.prisma.userRecord.update({
      where: { username },
      data: { password: encryptPassword },
    });
    return username;
  }

  async changeUsername(newUsername: string, username: string) {
    try {
      const result = await this.prisma.userRecord.update({
        where: { username },
        data: { username: newUsername },
      });
      return username;
    } catch (err) {
      return { code: 1009, message: '用户名已存在' };
    }
  }

  async changeSecret(secret: string) {
    const result = await this.prisma.commKv.upsert({
      where: { key: 'secret' },
      update: {
        value: secret,
      },
      create: {
        key: 'secret',
        value: secret,
      },
    });
    return result;
  }

  async getSecret() {
    const result = await this.prisma.commKv.findFirst({
      where: { key: 'secret' },
      select: { value: true },
    });
    return { secret: result.value };
  }

  async addComponentInfo(addInfo) {
    const result = await this.prisma.commKv.upsert({
      where: { key: 'redirect_uri' },
      update: { value: addInfo.redirectUrl },
      create: {
        key: 'redirect_uri',
        value: addInfo.redirectUrl,
      },
    });
    return result;
  }

  parseAuthorizerInfo(appInfo) {
    const funcInfo = appInfo.authorization_info.func_info
      .map((item: any) => {
        if (item?.confirm_info) {
          const id =
            item.confirm_info.already_confirm > 0
              ? item.funcscope_category.id
              : 0;
          return id;
        }

        return item.funcscope_category.id;
      })
      .filter((item: number) => item > 0)
      .join(',');
    const result = {
      id: appInfo.authorization_info.authorizer_appid,
      appid: appInfo.authorization_info.authorizer_appid,
      userName: appInfo.authorizer_info.user_name,
      nickName: appInfo.authorizer_info.nick_name,
      appType: appInfo.authorizer_info.MiniProgramInfo ? 0 : 1,
      serviceType: appInfo.authorizer_info.service_type_info.id,
      authTime: appInfo.authorization_info?.auth_time,
      principalName: appInfo.authorizer_info.principal_name,
      registerType: appInfo.authorizer_info.register_type || 0,
      accountStatus: appInfo.authorizer_info.account_status,
      basicConfig: appInfo.authorizer_info.basic_config,
      verifyInfo: appInfo.authorizer_info.verify_type_info.id,
      refreshToken: appInfo.authorization_info.authorizer_refresh_token,
      qrcodeUrl: appInfo.authorizer_info.qrcode_url,
      headImg: appInfo.authorizer_info.head_img,
      funcInfo,
      accessToken: appInfo.authorization_info.authorizer_access_token,
      expiresIn: appInfo.authorization_info.expires_in,
    };
    return result;
  }

  async getAuthorizerList(params) {
    const { offset, limit } = params;
    const { list, total_count } = await this.wx.getAuthorizerList(
      limit,
      offset,
    );
    let records = list || [];
    let total = total_count || 0;
    if (list?.length > 0) {
      const authorIds = list.map((item: any) => {
        const appid = item.authorizer_appid;
        return this.wx.getAuthorizerInfo(appid);
      });

      const result = await Promise.all(authorIds);
      records = result.map(this.parseAuthorizerInfo);
      this.addAuthorizerInfo(records);
    }

    return { records, total };
  }

  async addAuthorizerInfo(records) {
    const now = getDateTime(Date.now());
    const addToken = records
      .filter((item: any) => item.accessToken)
      .map((item: any) => {
        const addItem = this.prisma.wxToken.upsert({
          where: {
            appid_type: { appid: item.appid, type: 'authorizer_access_token' },
          },
          update: {
            token: item.accessToken,
            expire_time: timeAdd(now, parseInt(item.expiresIn), 's'),
          },
          create: {
            appid: item.appid,
            token: item.accessToken,
            type: 'authorizer_access_token',
            expire_time: timeAdd(now, parseInt(item.expiresIn), 's'),
          },
        });

        return addItem;
      });
    const addRecords = records.map((item: any) => {
      const addItem = this.prisma.authorizer.upsert({
        where: { appid: item.appid },
        update: {
          app_type: item.appType,
          service_type: item.serviceType,
          nickname: item.nickName,
          username: item.userName,
          headimg: item.headImg,
          qrcodeurl: item.qrcodeUrl,
          principalname: item.principalName,
          refreshtoken: item.refreshToken,
          funcinfo: item.funcInfo,
          verifyinfo: item.verifyInfo,
          auth_time: now,
        },
        create: {
          app_type: item.appType,
          appid: item.appid,
          service_type: item.serviceType,
          nickname: item.nickName,
          username: item.userName,
          headimg: item.headImg,
          qrcodeurl: item.qrcodeUrl,
          principalname: item.principalName,
          refreshtoken: item.refreshToken,
          funcinfo: item.funcInfo,
          verifyinfo: item.verifyInfo,
          auth_time: now,
        },
      });

      return addItem;
    });

    const result = await this.prisma.$transaction([...addRecords, ...addToken]);
    return result;
  }

  async getAuthorizerAccessToken(appid: string) {
    const result = await this.prisma.wxToken.findUnique({
      where: { appid_type: { appid, type: 'authorizer_access_token' } },
      select: { token: true },
    });
    if (result?.token) {
      return result.token;
    } else {
      throw new CustomException('该appid没有返回有效token', 1009);
    }
  }

  async getDevWeappList(params) {
    const { offset = 0, limit = 15 } = params;
    const result = await this.prisma.authorizer.findMany({
      where: { app_type: 0, funcinfo: { contains: '18' } },
      take: parseInt(limit),
      skip: parseInt(offset),
    });
    return { records: result, total: result?.length };
  }

  async getComponentVerifyTicket() {
    const result = await this.prisma.commKv.findFirst({
      where: { key: 'ticket' },
      select: { value: true },
    });
    return { ticket: result.value };
  }

  async getComponentAccessToken() {
    const result = await this.prisma.wxToken.findFirst({
      where: { type: 'component_access_token' },
      select: { token: true },
    });
    return { token: result.token };
  }

  async getWxComponentRecords(params) {
    let { limit = 15, offset = 0, startTime, endTime, infoType } = params;

    startTime = startTime
      ? getDateTime(new Date(startTime * 1000))
      : getDateTime(new Date('2022-11-24 00:00:00'));
    endTime = endTime
      ? getDateTime(new Date(endTime * 1000))
      : getDateTime(new Date());

    let condition = {
      receive_time: { gte: startTime, lte: endTime },
      info_type: infoType,
    };
    if (!infoType) {
      delete condition.info_type;
    }

    const result = await this.prisma.wxCallbackComponentRecord.findMany({
      where: condition,
      take: parseInt(limit),
      skip: parseInt(offset),
    });
    const records = result?.length
      ? result.map((item: any) => ({
          receiveTime: item.receive_time,
          infoType: item.info_type,
          postBody: item.post_body,
        }))
      : [];
    return { records, total: records.length };
  }

  async getWxBizRecords(params) {
    let {
      limit = 15,
      offset = 0,
      startTime,
      endTime,
      appid,
      event,
      msgType,
    } = params;

    startTime = startTime
      ? getDateTime(new Date(startTime * 1000))
      : getDateTime(new Date('2022-11-24 00:00:00'));
    endTime = endTime
      ? getDateTime(new Date(endTime * 1000))
      : getDateTime(new Date());

    let condition = {
      receive_time: { gte: startTime, lte: endTime },
      appid,
      event,
      msg_type: msgType,
    };
    if (!appid) {
      delete condition.appid;
    }

    if (!event) {
      delete condition.event;
    }

    if (!msgType) {
      delete condition.msg_type;
    }

    const result = await this.prisma.wxCallbackBizRecord.findMany({
      where: condition,
      take: parseInt(limit),
      skip: parseInt(offset),
    });
    const records = result?.length
      ? result.map((item: any) => ({
          receiveTime: item.receive_time,
          event: item.event,
          msgType: item.msg_type,
          appid: item.appid,
          infoType: item.info_type,
          postBody: item.post_body,
        }))
      : [];
    return { records, total: records.length };
  }

  async getProxyConfig() {
    const result = await this.prisma.commKv.findMany({
      where: { key: { in: ['proxy_state', 'proxy_port'] } },
      select: { key: true, value: true },
    });
    const open =
      'open' == result.find((item: any) => item.key == 'proxy_state')?.value;
    const port =
      result.find((item: any) => item.key == 'proxy_port')?.value || 8082;
    return { open, port };
  }

  async updateProxyConfig(open, port) {
    open = open ? 'open' : 'close';
    port = String(port);
    const updateState = this.prisma.commKv.upsert({
      where: { key: 'proxy_state' },
      update: { value: open },
      create: {
        key: 'proxy_state',
        value: open,
      },
    });
    const updatePort = this.prisma.commKv.upsert({
      where: { key: 'proxy_port' },
      update: { value: port },
      create: {
        key: 'proxy_port',
        value: port,
      },
    });
    const result = await this.prisma.$transaction([updatePort, updateState]);
    return result;
  }

  async getCallbackProxyRuleList(type, offset, limit) {
    const result = await this.prisma.wxCallbackRule.findMany({
      where: { type: parseInt(type) },
      take: parseInt(limit),
      skip: parseInt(offset),
    });
    const rules = result?.length
      ? result.map((item: any) => ({
          name: item.name,
          msgType: item.msg_type,
          event: item.event,
          open: item.open,
          updateTime: item.update_time,
          infoType: item.info_type,
          type: item.type,
          data: JSON.parse(item.post_body),
          id: item.id,
        }))
      : [];
    return { rules };
  }

  async addCallbackProxyRule(params) {
    const {
      name,
      type,
      event = '',
      msgType = '',
      info = '',
      open,
      data,
    } = params;
    try {
      const result = await this.prisma.wxCallbackRule.create({
        data: {
          name,
          type,
          msg_type: msgType,
          event,
          info_type: info,
          info,
          open,
          post_body: JSON.stringify(data),
        },
      });
      return result;
    } catch (err) {
      console.log(err);
      throw new CustomException('已存在相同规则', 1001);
    }
  }

  async updateCallbackProxyRule(id, data) {
    try {
      const result = await this.prisma.wxCallbackRule.update({
        where: { id: parseInt(id) },
        data,
      });
      return result;
    } catch (err) {
      console.log(err);
      throw new CustomException('更新失败，请再试试', 1001);
    }
  }

  async deleteCallbackProxyRule(id) {
    try {
      const result = await this.prisma.wxCallbackRule.delete({
        where: { id: parseInt(id) },
      });
      return result;
    } catch (err) {
      console.log(err);
      throw new CustomException('删除失败，请再试试', 1001);
    }
  }
}
