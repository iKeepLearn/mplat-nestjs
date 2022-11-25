import { Test, TestingModule } from '@nestjs/testing';
import { AuthpageController } from './authpage.controller';
import { AuthpageService } from './authpage.service';

describe('AuthpageController', () => {
  let controller: AuthpageController;

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      controllers: [AuthpageController],
      providers: [AuthpageService],
    }).compile();

    controller = module.get<AuthpageController>(AuthpageController);
  });

  it('should be defined', () => {
    expect(controller).toBeDefined();
  });
});
