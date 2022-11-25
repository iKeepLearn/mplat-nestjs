import { Injectable } from '@nestjs/common';
import { PrismaService } from 'src/prisma/prisma.service';
import { WxService } from 'src/wx/wx.service';

@Injectable()
export class AuthpageService {
  constructor(private prisma: PrismaService, private wx: WxService) {}
  async getComponentInfo() {
    const result = await this.prisma.commKv.findMany({
      where: { key: { in: ['appid', 'redirect_uri'] } },
      select: { key: true, value: true },
    });
    const appid = result.find((item: any) => item.key == 'appid').value;
    const redirectUrl = result.find(
      (item: any) => item.key == 'redirect_uri',
    ).value;
    return { appid, redirectUrl };
  }

  async getPreauthCode() {
    const result = await this.wx.getPreauthCode();

    return { preAuthCode: result.pre_auth_code };
  }
}
