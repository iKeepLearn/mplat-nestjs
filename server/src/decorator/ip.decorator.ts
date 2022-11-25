import { SetMetadata } from '@nestjs/common';

export const OnlyIp = (...ips: string[]) => SetMetadata('ips', ips);