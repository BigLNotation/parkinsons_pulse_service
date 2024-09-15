resource "aws_security_group" "public_sg" {
  name   = "biglnotation_public_sg"
  vpc_id = aws_vpc.vpc.id

  ingress {
    description = "Helloworld test"
    from_port = 4444
    to_port = 4444
    protocol = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    description = "HTTPS"
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    description = "HTTP"
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port = 0
    to_port   = 0
    protocol  = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
}

resource "aws_security_group" "user_db_sg" {
  name = "biglnotation_user_db_sg"
  vpc_id = aws_vpc.vpc.id

  ingress {
    description = "mongo db port"
    from_port = 27017
    to_port = 27017
    protocol = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }
}
