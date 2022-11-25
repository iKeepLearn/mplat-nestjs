import { Test, TestingModule } from '@nestjs/testing';
import { AuthpageService } from './authpage.service';

describe('AuthpageService', () => {
  let service: AuthpageService;

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      providers: [AuthpageService],
    }).compile();

    service = module.get<AuthpageService>(AuthpageService);
  });

  it('should be defined', () => {
    expect(service).toBeDefined();
  });
});
