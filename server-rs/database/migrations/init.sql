-- Authorizer 授权账号
CREATE TABLE authorizer (
    id SERIAL PRIMARY KEY,
    appid VARCHAR NOT NULL UNIQUE,
    app_type INTEGER NOT NULL,
    service_type INTEGER NOT NULL,
    nickname VARCHAR NOT NULL,
    username VARCHAR NOT NULL,
    headimg VARCHAR NOT NULL,
    qrcodeurl VARCHAR NOT NULL,
    principalname VARCHAR NOT NULL,
    refreshtoken VARCHAR NOT NULL,
    funcinfo VARCHAR NOT NULL,
    verifyinfo INTEGER NOT NULL,
    auth_time TIMESTAMP NOT NULL
);

-- WxCallbackComponentRecord 第三方授权事件的记录
CREATE TABLE wx_callback_component_record (
    id SERIAL PRIMARY KEY,
    receive_time TIMESTAMP NOT NULL,
    create_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    info_type VARCHAR NOT NULL,
    post_body VARCHAR NOT NULL
);

CREATE INDEX idx_wx_callback_component_record_receive_time ON wx_callback_component_record(receive_time);

-- WxCallbackBizRecord 小程序授权事件记录
CREATE TABLE wx_callback_biz_record (
    id SERIAL PRIMARY KEY,
    receive_time TIMESTAMP NOT NULL,
    create_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    appid VARCHAR NOT NULL,
    to_user_name VARCHAR NOT NULL,
    msg_type VARCHAR NOT NULL,
    event VARCHAR NOT NULL,
    info_type VARCHAR NOT NULL,
    post_body VARCHAR NOT NULL
);

CREATE INDEX idx_wx_callback_biz_record_receive_time ON wx_callback_biz_record(receive_time);

-- WxCallbackRule 回调消息转发规则
CREATE TABLE wx_callback_rule (
    id SERIAL PRIMARY KEY,
    update_time TIMESTAMP NOT NULL,
    create_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    name VARCHAR NOT NULL,
    type INTEGER NOT NULL,
    msg_type VARCHAR NOT NULL,
    event VARCHAR NOT NULL,
    info_type VARCHAR NOT NULL,
    info VARCHAR NOT NULL,
    open INTEGER NOT NULL,
    post_body VARCHAR NOT NULL,
    CONSTRAINT unique_wx_callback_rule UNIQUE (info_type, msg_type, event)
);

-- HttpProxyConfig http转发配置
CREATE TABLE http_proxy_config (
    id SERIAL PRIMARY KEY,
    port INTEGER NOT NULL,
    path VARCHAR NOT NULL
);

-- UserRecord 用户信息
CREATE TABLE user_record (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL UNIQUE,
    password VARCHAR NOT NULL,
    create_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_time TIMESTAMP NOT NULL
);

-- WxToken 微信相关的token
CREATE TABLE wx_token (
    id SERIAL PRIMARY KEY,
    create_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_time TIMESTAMP NOT NULL,
    type VARCHAR NOT NULL,
    appid VARCHAR NOT NULL,
    token VARCHAR NOT NULL,
    expire_time TIMESTAMP NOT NULL,
    CONSTRAINT unique_wx_token_appid_type UNIQUE (appid, type)
);

CREATE INDEX idx_wx_token_appid_type ON wx_token(appid, type);

-- CommKv 通用的kv
CREATE TABLE comm_kv (
    id SERIAL PRIMARY KEY,
    create_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_time TIMESTAMP NOT NULL,
    key VARCHAR NOT NULL UNIQUE,
    value VARCHAR NOT NULL
);

-- Counter 计数器
CREATE TABLE counter (
    id SERIAL PRIMARY KEY,
    create_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_time TIMESTAMP NOT NULL,
    key VARCHAR NOT NULL UNIQUE,
    value INTEGER NOT NULL
);

-- 创建更新 update_time 的函数
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.update_time = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 为每个有 update_time 的表创建触发器
-- wx_callback_rule
CREATE TRIGGER update_wx_callback_rule_updated_at 
    BEFORE UPDATE ON wx_callback_rule 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- user_record
CREATE TRIGGER update_user_record_updated_at 
    BEFORE UPDATE ON user_record 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- wx_token
CREATE TRIGGER update_wx_token_updated_at 
    BEFORE UPDATE ON wx_token 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- comm_kv
CREATE TRIGGER update_comm_kv_updated_at 
    BEFORE UPDATE ON comm_kv 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- counter
CREATE TRIGGER update_counter_updated_at 
    BEFORE UPDATE ON counter 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();