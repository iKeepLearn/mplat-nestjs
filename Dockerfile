
FROM node:latest AS builder
WORKDIR /app


COPY ./server/package.json ./server/yarn.lock ./server/tsconfig.json ./
COPY ./server/prisma ./prisma
COPY ./server/.env ./.env
COPY ./server/src ./src

RUN yarn && yarn build && npx prisma generate



FROM node:latest AS clientbuilder

WORKDIR /app

COPY ./client/package.json ./client/yarn.lock ./client/tsconfig.json ./client/vite.config.ts ./
COPY ./client/index.html ./index.html
COPY ./client/src ./src
RUN yarn && yarn build 





FROM node:16.17.0-bullseye-slim AS runner

WORKDIR /app

ENV NODE_ENV=production
ENV PATH /app/node_modules/.bin:$PATH
USER node


COPY --chown=node:node --from=builder /app/node_modules ./node_modules
COPY --chown=node:node --from=builder /app/dist ./dist
COPY --chown=node:node --from=builder /app/prisma ./prisma
COPY --chown=node:node --from=builder /app/.env ./.env
COPY --chown=node:node --from=clientbuilder /app/dist ./client/dist


EXPOSE 3001


CMD ["node","dist/src/main"]