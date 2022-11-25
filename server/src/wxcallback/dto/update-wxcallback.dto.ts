import { PartialType } from '@nestjs/mapped-types';
import { CreateWxcallbackDto } from './create-wxcallback.dto';

export class UpdateWxcallbackDto extends PartialType(CreateWxcallbackDto) {}
