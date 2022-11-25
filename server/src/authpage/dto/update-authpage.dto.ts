import { PartialType } from '@nestjs/mapped-types';
import { CreateAuthpageDto } from './create-authpage.dto';

export class UpdateAuthpageDto extends PartialType(CreateAuthpageDto) {}
