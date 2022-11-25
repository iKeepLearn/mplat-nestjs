import { HttpModule } from '@nestjs/axios';
import { Global, Module } from '@nestjs/common';
import { WxService } from './wx.service';

@Global()
@Module({
  imports: [HttpModule],
  providers: [WxService],
  exports: [WxService],
})
export class WxModule {}
