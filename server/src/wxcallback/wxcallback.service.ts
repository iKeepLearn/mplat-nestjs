import { Injectable } from '@nestjs/common';
import { PrismaService } from 'src/prisma/prisma.service';
import { getDateTime } from 'src/utils';
import { WxService } from 'src/wx/wx.service';
import { XMLParser } from 'fast-xml-parser'

@Injectable()
export class WxcallbackService {
  constructor(private wx: WxService, private prisma: PrismaService) { }

  async getComponentVerifyTicke(detail, query) {
    if (detail) {
      const { timestamp, nonce, msg_signature } = query;
      const encryptedMsg = Buffer.from(detail, 'base64').toString();
      const options = {
        ignoreAttributes: false
      };

      const xmlParser = new XMLParser(options);
      const jsonObj = xmlParser.parse(encryptedMsg);
      const { Encrypt: encrypt } = jsonObj.xml

      const signature = this.wx.genSign({ timestamp, nonce, encrypt });

      if (signature === msg_signature) {

        const xml = this.wx.decode(encrypt);
        const xmlBody = xmlParser.parse(xml)
        console.log({ xmlBody })
        const { AppId, CreateTime, InfoType } = xmlBody.xml
        const createTime = getDateTime(CreateTime * 1000)
        const addRecord = await this.prisma.wxCallbackComponentRecord
          .create({
            data: {
              receive_time: createTime,
              info_type: InfoType,
              post_body: JSON.stringify(xmlBody.xml),
            },
          });

        if (InfoType == 'component_verify_ticket') {
          const ticket = xmlBody.xml.ComponentVerifyTicket

          const updateTicket = await this.prisma.commKv.upsert({
            where: { key: 'ticket' },
            update: { value: ticket },
            create: { key: 'ticket', value: ticket },
          });


        }
      }
    }

    return 'success';
  }

  async getAuthInfo(detail, query, appid) {
    console.log('get auth info', { detail, query, appid });
    return 'success';
  }
}
