import { Injectable } from '@nestjs/common';
import { ConfigService } from '@nestjs/config';
import { PrismaClient } from '@prisma/client';

@Injectable()
export class PrismaService extends PrismaClient {
  constructor(config: ConfigService) {
    super({
      datasources: {
        db: {
          //url: process.env.NODE_ENV == 'production' ? config.get('DATABASE_IN_DOCKER_URL') : config.get('DATABASE__URL'),
          url: config.get('DATABASE_URL'),
        },
      },
    });
  }
}
