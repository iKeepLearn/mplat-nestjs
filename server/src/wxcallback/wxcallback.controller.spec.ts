import { Test, TestingModule } from '@nestjs/testing';
import { WxcallbackController } from './wxcallback.controller';
import { WxcallbackService } from './wxcallback.service';

describe('WxcallbackController', () => {
  let controller: WxcallbackController;

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      controllers: [WxcallbackController],
      providers: [WxcallbackService],
    }).compile();

    controller = module.get<WxcallbackController>(WxcallbackController);
  });

  it('should be defined', () => {
    expect(controller).toBeDefined();
  });
});
