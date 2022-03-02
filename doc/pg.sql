CREATE TABLE "sys_menu" (
  "id" numeric(24) primary key,
  "name" varchar(100) COLLATE "pg_catalog"."default" NOT NULL,
  "show_flag" numeric(1) NOT NULL,
  "create_date" timestamp(6) NOT NULL,
  "create_by" varchar(64) COLLATE "pg_catalog"."default" NOT NULL,
  "code" varchar(8) COLLATE "pg_catalog"."default",
  "parent_code" varchar(8) COLLATE "pg_catalog"."default"
);

COMMENT ON TABLE "public"."sys_menu" IS '系统::菜单表';

INSERT INTO "sys_menu"("id", "name", "show_flag", "create_date", "create_by", "code", "parent_code") VALUES ('2101061645320080100002', '主页', '1', '2020-11-27 03:09:21.714105', 'sys', '01', NULL);
INSERT INTO "sys_menu"("id", "name", "show_flag", "create_date", "create_by", "code", "parent_code") VALUES ('2101061651380080100003', '数据导入', '1', '2020-11-27 03:09:24.25396', 'sys', '02', NULL);
INSERT INTO "sys_menu"("id", "name", "show_flag", "create_date", "create_by", "code", "parent_code") VALUES ('2101061652550080100004', '截图&预览', '1', '2020-12-09 11:44:04.478717', 'sys', '03', NULL);
INSERT INTO "sys_menu"("id", "name", "show_flag", "create_date", "create_by", "code", "parent_code") VALUES ('2101061654540080100005', '系统配置', '1', '2020-11-27 03:09:29.581216', 'sys', '04', NULL);
INSERT INTO "sys_menu"("id", "name", "show_flag", "create_date", "create_by", "code", "parent_code") VALUES ('2101291855050080100000', '系统配置', '1', '2020-11-27 03:09:40.177621', 'sys', '0401', '04');
INSERT INTO "sys_menu"("id", "name", "show_flag", "create_date", "create_by", "code", "parent_code") VALUES ('2101291856150080100001', '基础管理', '1', '2020-11-27 03:09:36.788131', 'sys', '0402', '04');
INSERT INTO "sys_menu"("id", "name", "show_flag", "create_date", "create_by", "code", "parent_code") VALUES ('2101291857220080100002', '用户配置', '1', '2020-11-27 03:09:42.242687', 'sys', '040101', '0401');
INSERT INTO "sys_menu"("id", "name", "show_flag", "create_date", "create_by", "code", "parent_code") VALUES ('2101291857440080100003', '菜单配置', '1', '2020-11-27 03:09:44.198666', 'sys', '040103', '0401');
INSERT INTO "sys_menu"("id", "name", "show_flag", "create_date", "create_by", "code", "parent_code") VALUES ('2101291857440080100013', '角色分配', '1', '2020-11-27 03:09:47.713889', 'sys', '040102', '0401');
INSERT INTO "sys_menu"("id", "name", "show_flag", "create_date", "create_by", "code", "parent_code") VALUES ('2101291858100080100004', '字典配置', '1', '2020-11-27 03:09:49.643454', 'sys', '040201', '0402');
INSERT INTO "sys_menu"("id", "name", "show_flag", "create_date", "create_by", "code", "parent_code") VALUES ('2101291858450080100005', '日志配置', '1', '2020-11-27 03:09:52.519771', 'sys', '040202', '0402');
