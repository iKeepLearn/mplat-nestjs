import {
  Controller,
  Get,
  Post,
  Body,
  Patch,
  Param,
  Delete,
  UseGuards,
  Query,
  Put,
} from '@nestjs/common';
import { GetUser } from 'src/decorator';
import { JwtGuard } from 'src/guard';
import { AdminService } from './admin.service';
import { ChangePwdDto } from './dto/change-pwd.dto';

@UseGuards(JwtGuard)
@Controller('admin')
export class AdminController {
  constructor(private readonly adminService: AdminService) {}

  @Post('userpwd')
  changePassword(@Body() detail: ChangePwdDto, @GetUser() user: any) {
    return this.adminService.changePassword(detail, user.username);
  }

  @Post('username')
  changeUsername(@Body('username') username: string, @GetUser() user: any) {
    return this.adminService.changeUsername(username, user.username);
  }

  @Post('secret')
  changeSecret(@Body('secret') secret: string) {
    return this.adminService.changeSecret(secret);
  }

  @Get('secret')
  getSecret() {
    return this.adminService.getSecret();
  }

  @Get('authorizer-list')
  getAuthorizerList(@Query() params: any) {
    return this.adminService.getAuthorizerList(params);
  }

  @Post('componentinfo')
  addComponentInfo(@Body() addInfo: any) {
    return this.adminService.addComponentInfo(addInfo);
  }

  @Get('authorizer-access-token')
  getAuthorizerAccessToken(@Query('appid') appid: string) {
    return this.adminService.getAuthorizerAccessToken(appid);
  }

  @Get('dev-weapp-list')
  getDevWeappList(@Query() params: any) {
    return this.adminService.getDevWeappList(params);
  }

  @Get('ticket')
  getComponentVerifyTicket() {
    return this.adminService.getComponentVerifyTicket();
  }

  @Get('component-access-token')
  getComponentAccessToken() {
    return this.adminService.getComponentAccessToken();
  }

  @Get('wx-component-records')
  getWxComponentRecords(@Query() params: any) {
    return this.adminService.getWxComponentRecords(params);
  }

  @Get('wx-biz-records')
  getWxBizRecords(@Query() params: any) {
    return this.adminService.getWxBizRecords(params);
  }

  @Get('proxy')
  getProxyConfig() {
    return this.adminService.getProxyConfig();
  }

  @Post('proxy')
  updateProxyConfig(@Body() params: any) {
    const { open, port } = params;
    return this.adminService.updateProxyConfig(open, port);
  }

  @Get('callback-proxy-rule-list')
  getCallbackProxyRuleList(@Query() params: any) {
    const { type, offset, limit } = params;
    return this.adminService.getCallbackProxyRuleList(type, offset, limit);
  }

  @Put('callback-proxy-rule')
  addCallbackProxyRule(@Body() params: any) {
    return this.adminService.addCallbackProxyRule(params);
  }

  @Post('callback-proxy-rule')
  updateCallbackProxyRule(@Body() params: any) {
    const { id, data } = params;
    return this.adminService.updateCallbackProxyRule(id, data);
  }

  @Delete('callback-proxy-rule')
  deleteCallbackProxyRule(@Query('id') id: number) {
    return this.adminService.deleteCallbackProxyRule(id);
  }
}
