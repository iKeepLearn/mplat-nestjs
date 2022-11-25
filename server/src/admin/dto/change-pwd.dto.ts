// import { Transform } from 'class-transformer';
import { IsMobilePhone, IsNotEmpty, IsString } from 'class-validator';

export class ChangePwdDto {
    @IsString()
    @IsNotEmpty({ message: '请确认旧密码是否正确' })
    oldPassword: string;

    @IsString()
    @IsNotEmpty({ message: '请确认密码是否正确' })
    password: string;
}
