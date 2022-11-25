import { HttpException, HttpStatus } from '@nestjs/common'

export class CustomException extends HttpException {
    constructor(options: any, statusCode?: HttpStatus) {
        super(options, statusCode || HttpStatus.INTERNAL_SERVER_ERROR)
    }
}