// import { Transform } from 'class-transformer';
import { IsMobilePhone, IsNotEmpty, IsString } from 'class-validator';

export class AuthDto {
  @IsString()
  @IsNotEmpty({ message: '请确认username是否正确' })
  username: string;

  @IsString()
  @IsNotEmpty({ message: '请确认password是否正确' })
  password: string;
}
