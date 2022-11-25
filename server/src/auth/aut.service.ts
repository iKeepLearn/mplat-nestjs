import { Injectable } from '@nestjs/common';
import { ConfigService } from '@nestjs/config';
import { JwtService } from '@nestjs/jwt';
import * as bcrypt from 'bcrypt';
import { CustomException } from 'src/exceptions/custom.exception';
import { PrismaService } from 'src/prisma/prisma.service';
@Injectable()
export class AuthService {
  constructor(
    private prisma: PrismaService,
    private jwt: JwtService,
    private config: ConfigService,
  ) { }

  async hashPassword(password: string) {
    const saltOrRounds = parseInt(this.config.get("BCRYPT_SALT_LENGTH"));
    return await bcrypt.hash(password, saltOrRounds);
  }

  async comparePassword(password: string, hash) {
    return await bcrypt.compare(password, hash)
  }

  async signin(dto) {
    const { password, username } = dto
    const user = await this.prisma.userRecord.findUnique({
      where: { username }, select: { password: true }
    })

    if (!user) {
      throw new CustomException("未注册", 1009)
    }

    const passwordIsMatch = await this.comparePassword(password, user.password)

    if (user && !passwordIsMatch) {
      throw new CustomException("密码不正确", 1009)
    }

    const { access_token } = await this.signToken(username)
    return { jwt: access_token }

  }

  async signToken(username: string) {
    const payload = {
      sub: username,
    };

    const secret = this.config.get('JWT_SECRET');
    const access_token = await this.jwt.signAsync(payload, {
      expiresIn: '1d',
      secret,
    });
    return { access_token };
  }

  async signup(dto) {
    const { password, username } = dto
    const encryptPassword = await this.hashPassword(password)

    try {
      const user = await this.prisma.userRecord.create({
        data: { username, password: encryptPassword }
      })
      return { username }
    } catch (err) {
      throw new CustomException("该用户名已存在", 1009)
    }



  }
}
