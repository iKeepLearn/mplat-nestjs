import { MiddlewareConsumer, Module, NestModule, RequestMethod } from '@nestjs/common';
import { ConfigModule } from '@nestjs/config';
import { APP_FILTER, APP_INTERCEPTOR } from '@nestjs/core';
import { AuthModule } from './auth/auth.module';
import { AllExceptionsFilter } from './filters/error.filter';
import { TransformInterceptor } from './interceptor/transform.interceptor';
import { PrismaModule } from './prisma/prisma.module';
import { AdminModule } from './admin/admin.module';
import { AuthpageModule } from './authpage/authpage.module';
import { WxModule } from './wx/wx.module';
import { WxcallbackModule } from './wxcallback/wxcallback.module';
import { join } from 'path';
import { ServeStaticModule } from '@nestjs/serve-static';
import { applyRawBodyOnlyTo } from '@golevelup/nestjs-webhooks';



@Module({
  imports: [
    ConfigModule.forRoot({ isGlobal: true }),
    ServeStaticModule.forRoot({
      rootPath: join(__dirname, '../../..', 'client/dist'),
      exclude: ['/api*'],
    }),
    AuthModule,
    PrismaModule,
    AdminModule,
    AuthpageModule,
    WxModule,
    WxcallbackModule,
  ],
  controllers: [],
  providers: [
    {
      provide: APP_FILTER,
      useClass: AllExceptionsFilter,
    },
    {
      provide: APP_INTERCEPTOR,
      useClass: TransformInterceptor
    }
  ],
})
export class AppModule implements NestModule {
  configure(consumer: MiddlewareConsumer) {
    applyRawBodyOnlyTo(consumer, {
      method: RequestMethod.ALL,
      path: '*wxcallback*',
    });
  }
}
