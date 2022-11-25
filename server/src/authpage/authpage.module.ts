import { Module } from '@nestjs/common';
import { AuthpageService } from './authpage.service';
import { AuthpageController } from './authpage.controller';

@Module({
  controllers: [AuthpageController],
  providers: [AuthpageService],
})
export class AuthpageModule {}
