resource "aws_security_group" "public_sg" {
  name   = "biglnotation_public_sg"
  vpc_id = aws_vpc.vpc.id

  ingress {
    description = "Server main port"
    from_port   = 4444
    to_port     = 4444
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
}

resource "aws_security_group" "user_db_sg" {
  name   = "biglnotation_user_db_sg"
  vpc_id = aws_vpc.vpc.id

  ingress {
    description = "mongo db port"
    from_port   = 27017
    to_port     = 27017
    protocol    = "tcp"
  }
}

resource "aws_security_group" "user_db_connection_sg" {
  name   = "biglnotation_connect_user_db_sg"
  vpc_id = aws_vpc.vpc.id

  egress {
    from_port       = 27017
    to_port         = 27017
    protocol        = "tcp"
    security_groups = [aws_security_group.user_db_sg.id]
  }
}

resource "aws_security_group" "pp_service_alb_sg" {
  vpc_id                 = aws_vpc.vpc.id
  name                   = "pp-service-sg-alb"
  description            = "Security group for alb relating to pp service"
  revoke_rules_on_delete = true
}

resource "aws_security_group_rule" "alb_http_ingress" {
  type              = "ingress"
  from_port         = 80
  to_port           = 80
  protocol          = "TCP"
  description       = "Allow http inbound traffic from internet"
  security_group_id = aws_security_group.pp_service_alb_sg.id
  cidr_blocks       = ["0.0.0.0/0"]
}

resource "aws_security_group_rule" "alb_https_ingress" {
  type              = "ingress"
  from_port         = 443
  to_port           = 443
  protocol          = "TCP"
  description       = "Allow https inbound traffic from internet"
  security_group_id = aws_security_group.pp_service_alb_sg.id
  cidr_blocks       = ["0.0.0.0/0"]
}

resource "aws_security_group_rule" "alb_egress" {
  type              = "egress"
  from_port         = 0
  to_port           = 0
  protocol          = "-1"
  description       = "Allow outbound traffic from alb"
  security_group_id = aws_security_group.pp_service_alb_sg.id
  cidr_blocks       = ["0.0.0.0/0"]
}