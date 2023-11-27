DROP DATABASE IF EXISTS cheery;
CREATE DATABASE cheery CHARACTER SET 'utf8mb4' COLLATE 'utf8mb4_unicode_ci';

USE cheery;

DROP TABLE IF EXISTS `member`;
CREATE TABLE `member` (
	`member_id` BIGINT UNSIGNED PRIMARY KEY COMMENT '用户id',
	`nickname` VARCHAR ( 64 ) COMMENT '昵称',
	`password` VARCHAR ( 32 ) COMMENT '密码',
	`email` VARCHAR ( 128 ) COMMENT '邮箱',
	`created_at` TIMESTAMP ( 3 ) DEFAULT CURRENT_TIMESTAMP ( 3 ) COMMENT '创建时间',
	`updated_at` TIMESTAMP ( 3 ) DEFAULT CURRENT_TIMESTAMP ( 3 ) ON UPDATE CURRENT_TIMESTAMP ( 3 ) COMMENT '更新时间',
	`deleted_at` TIMESTAMP ( 3 ) COMMENT '删除时间' 
);
INSERT INTO `member` (`member_id`, `nickname`, `password`, `email`) VALUES (1, "忘机", "123456", "wj@gmail.com");
INSERT INTO `member` (`member_id`, `nickname`, `password`, `email`) VALUES (2, "白鹤", "123456", "bh@gmail.com");
INSERT INTO `member` (`member_id`, `nickname`, `password`, `email`) VALUES (3, "花咲", "123456", "hx@gmail.com");
INSERT INTO `member` (`member_id`, `nickname`, `password`, `email`) VALUES (4, "明月", "123456", "my@gmail.com");
