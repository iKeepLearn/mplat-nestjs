import { BadRequestException, ExecutionContext, Injectable } from '@nestjs/common';
import { AuthGuard } from '@nestjs/passport';



@Injectable()
export class AdminOnlyGuard extends AuthGuard('jwt') {
    canActivate(context: ExecutionContext) {
        return super.canActivate(context)
    }

    handleRequest(error, authInfo, errInfo) {
        // console.log({ authInfo, errInfo })
        if (authInfo && authInfo.openid == 'o7csv5QQfZaKIO6VCROtTBQP5WCs') {
            return authInfo
        } else {
            throw error || new BadRequestException('未认证')
        }
    }
}


