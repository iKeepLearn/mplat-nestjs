import { Controller, Post, Body, Query, HttpCode, Param } from '@nestjs/common';
import { DataResponse } from 'src/decorator/data-response.decorator';
import { WxcallbackService } from './wxcallback.service';


@Controller('wxcallback')
export class WxcallbackController {
  constructor(private readonly wxcallbackService: WxcallbackService) { }

  @Post('component')
  @DataResponse("raw")
  @HttpCode(200)
  getComponentVerifyTicke(@Body() detail: any, @Query() query: any) {
    return this.wxcallbackService.getComponentVerifyTicke(detail, query)
  }

  @Post('biz/:appid')
  @DataResponse("raw")
  @HttpCode(200)
  getAuthInfo(@Body() detail: any, @Query() query: any, @Param("appid") appid: string) {
    return this.wxcallbackService.getAuthInfo(detail, query, appid)
  }
}
