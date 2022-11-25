import { Injectable, NestInterceptor, ExecutionContext, CallHandler } from '@nestjs/common';
import { Reflector } from '@nestjs/core';
import { Observable } from 'rxjs';
import { map } from 'rxjs/operators';

export interface Response<T> {
    data: T;
    code: number,
    message: string
}

@Injectable()
export class TransformInterceptor<T> implements NestInterceptor<T, Response<T>> {
    constructor(private reflector: Reflector) { }
    intercept(context: ExecutionContext, next: CallHandler): Observable<Response<T>> {
        const dataResponseType = this.reflector.get<string>('data-response', context.getHandler());
        return next.handle().pipe(map(data => {
            if (dataResponseType && dataResponseType == "raw") {
                return data
            } else {
                const code = data.code || 0
                const message = data.message || "请求成功"
                const result = { data, code, message }
                if (code != 0) {
                    result.data = message
                }
                return result
            }
        }));
    }
}