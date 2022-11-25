import {
  ExceptionFilter,
  Catch,
  ArgumentsHost,
  HttpException,
  HttpStatus,
} from '@nestjs/common';
import { HttpAdapterHost } from '@nestjs/core';

@Catch()
export class AllExceptionsFilter implements ExceptionFilter {
  constructor(private readonly httpAdapterHost: HttpAdapterHost) {}

  catch(exception: unknown, host: ArgumentsHost): void {
    const { httpAdapter } = this.httpAdapterHost;
    console.log(exception);
    const ctx = host.switchToHttp();

    let code = 1000;
    let message = '请求失败，请稍后再试';
    if (exception instanceof HttpException) {
      const exceptionRes = JSON.parse(JSON.stringify(exception.getResponse()));
      message = exceptionRes?.message || exceptionRes;
      code = exceptionRes?.code || exception.getStatus() || code;
    }

    const responseBody = {
      statusCode: HttpStatus.OK,
      code,
      timestamp: new Date().toISOString(),
      data: message,
      message,
      path: httpAdapter.getRequestUrl(ctx.getRequest()),
    };

    httpAdapter.reply(ctx.getResponse(), responseBody, HttpStatus.OK);
  }
}
