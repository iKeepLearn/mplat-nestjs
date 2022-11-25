import { PrismaClient } from '@prisma/client';
const prisma = new PrismaClient();
async function main() {
  const admin = prisma.userRecord.upsert({
    where: { username: 'admin' },
    update: {},
    create: {
      username: 'admin',
      password: '$2b$10$wns9TjsB/KFEQYXkgOadOuaBQ/adHowsfUe6l9FFZWbTF5hamZv1a',
    },
  });

  const commKv = prisma.commKv.createMany({
    data: [
      { key: 'secret', value: '第三方平台secret' },
      { key: 'appid', value: '第三方平台appid' },
      { key: 'token', value: '第三方平台解密token' },
      {
        key: 'encodingAESKey',
        value: '第三方平台解密key',
      },
      {
        key: 'ticket',
        value:
          '这个可以随便写，反正微信推过来后会更新',
      },
    ],
  });
  const result = await prisma.$transaction([admin, commKv]);
  console.log({ result });
}
main()
  .then(async () => {
    await prisma.$disconnect();
  })
  .catch(async (e) => {
    console.error(e);
    await prisma.$disconnect();
    process.exit(1);
  });
