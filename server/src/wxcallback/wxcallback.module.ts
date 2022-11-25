import { Module } from '@nestjs/common';
import { WxcallbackService } from './wxcallback.service';
import { WxcallbackController } from './wxcallback.controller';

@Module({
  controllers: [WxcallbackController],
  providers: [WxcallbackService]
})
export class WxcallbackModule {}
