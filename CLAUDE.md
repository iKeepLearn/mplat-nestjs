# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a NestJS backend for WeChat's third-party platform management tool (服务商微管家). It uses Prisma ORM with PostgreSQL and supports Docker containerization. The frontend is expected to be in `client/dist` (served statically).

## Key Commands

All commands should be run from the `server/` directory:

```bash
# Development
cd server
yarn start:dev          # Start server in watch mode
yarn start:debug        # Start server in debug mode with watch

# Build
yarn build              # Build the project

# Database
yarn prisma:dev:deploy  # Run Prisma migrations
yarn prisma db seed     # Seed the database (creates admin user and initial config)

# Code Quality
yarn lint               # Run ESLint with --fix
yarn format             # Run Prettier

# Testing
yarn test               # Run unit tests
yarn test:watch         # Run tests in watch mode
yarn test:cov           # Run tests with coverage
yarn test:e2e           # Run end-to-end tests

# Docker (from project root)
yarn server:up         # Start server container
yarn server:upd        # Start server container in background
yarn server:rm         # Stop and remove server container
yarn server:restart    # Restart server container
```

## Architecture

### Tech Stack
- **Framework**: NestJS 9.x
- **Language**: TypeScript 4.7
- **ORM**: Prisma 4.5
- **Database**: PostgreSQL
- **Auth**: JWT + Passport
- **Validation**: class-validator + class-transformer

### Module Structure
- **AuthModule** (`src/auth/`): JWT authentication, login, password change
- **AdminModule** (`src/admin/`): Admin user management
- **AuthpageModule** (`src/authpage/`): WeChat authorization page management
- **WxModule** (`src/wx/`): WeChat API integration service
- **WxcallbackModule** (`src/wxcallback/`): WeChat callback/event handling
- **PrismaModule** (`src/prisma/`): Database connection service

### Cross-Cutting Concerns
- **Global Pipes**: ValidationPipe with whitelist and transform enabled
- **Global Filters**: AllExceptionsFilter for error handling
- **Global Interceptors**: TransformInterceptor for response formatting
- **Guards**: JwtAuthGuard, AdminOnlyGuard, IpGuard
- **Decorators**: @GetUser(), @Ip(), @DataResponse(), @GetTime()

### Database Models
- `user_record`: Admin users (default: admin/admin)
- `authorizer`: WeChat authorized accounts
- `wx_callback_component_record`: Third-party platform event records
- `wx_callback_biz_record`: Mini program authorization event records
- `wx_callback_rule`: Callback message forwarding rules
- `wx_token`: WeChat tokens with expiration
- `comm_kv`: General key-value storage (WeChat platform config)
- `counter`: Counters
- `http_proxy_config`: HTTP proxy configuration

### Environment Variables (server/.env)
```
POSTGRES_USER=postgres
POSTGRES_PASSWORD=123
POSTGRES_DB=mplat
DATABASE_URL="postgresql://postgres:123@localhost:5432/mplat?schema=public"
BCRYPT_SALT_LENGTH=10
JWT_SECRET="fsal;fkdfdfnak"
```

## Initial Setup

1. Configure database in `server/.env`
2. Update WeChat platform config in `server/prisma/seed.ts`
3. Run `yarn install` in `server/`
4. Run `yarn prisma:dev:deploy` to run migrations
5. Run `yarn prisma db seed` to initialize database
6. Start with `yarn start:dev`

Default login: admin / admin
