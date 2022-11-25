import { SetMetadata } from '@nestjs/common';

export const DataResponse = (responseType: string) => SetMetadata('data-response', responseType);