import { ValidationPipe } from '@nestjs/common';
import { NestFactory } from '@nestjs/core';
import { repl } from '@nestjs/core';
import helmet from 'helmet'
import { AppModule } from './app.module';

async function bootstrap() {
  const app = await NestFactory.create(AppModule, { bodyParser: false });
  app.useGlobalPipes(new ValidationPipe({ whitelist: true, transform: true }));
  app.enableCors();
  app.setGlobalPrefix('api');
  await app.listen(3001);
}

async function localTest() {
  await repl(AppModule);
}
bootstrap();
//localTest();
