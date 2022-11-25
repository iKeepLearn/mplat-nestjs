import { Injectable, CanActivate, ExecutionContext } from '@nestjs/common';
import { Reflector } from '@nestjs/core';

@Injectable()
export class IpGuard implements CanActivate {
    constructor(private reflector: Reflector) { }

    canActivate(context: ExecutionContext): boolean {
        const ips = this.reflector.get<string[]>('ips', context.getHandler());
        const request = context.switchToHttp().getRequest();
        const ip = request.headers["cf-connecting-ip"];
        console.log(ip,ips)
        return ips.indexOf(ip) != -1;
    }
}