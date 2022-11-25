import { Test, TestingModule } from '@nestjs/testing';
import { WxcallbackService } from './wxcallback.service';

describe('WxcallbackService', () => {
  let service: WxcallbackService;

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      providers: [WxcallbackService],
    }).compile();

    service = module.get<WxcallbackService>(WxcallbackService);
  });

  it('should be defined', () => {
    expect(service).toBeDefined();
  });
});
