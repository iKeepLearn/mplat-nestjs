import { Controller, Get, Header } from '@nestjs/common';

import { AuthpageService } from './authpage.service';

@Controller('authpage')
export class AuthpageController {
  constructor(private readonly authpageService: AuthpageService) {}

  @Get('componentinfo')
  @Header('Cache-Control', 'none')
  getComponentInfo() {
    return this.authpageService.getComponentInfo();
  }

  @Get('preauthcode')
  @Header('Cache-Control', 'none')
  getPreauthCode() {
    return this.authpageService.getPreauthCode();
  }
}
