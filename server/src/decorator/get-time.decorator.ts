import { createParamDecorator, ExecutionContext } from '@nestjs/common';

export const GetTime = createParamDecorator(
  (data: string | undefined, ctx: ExecutionContext) => {
    const now = Date.now();
    return now;
  },
);
