resource "aws_docdb_cluster" "user_db" {
  cluster_identifier = "biglnotation-user-db"

  port = 27017

  master_username = "biglnotationDBdev"
  master_password = "WeAreTakingABigLOnThis"

  db_subnet_group_name = aws_docdb_subnet_group.pp_service.id
  vpc_security_group_ids = [aws_security_group.user_db_sg.id]
}

resource "aws_docdb_subnet_group" "pp_service" {
  name = "biglnotation_pp_service_user_db_subnet"
  subnet_ids = [aws_subnet.sn1_private.id, aws_subnet.sn2_private.id]
}

resource "aws_docdb_cluster_instance" "user_db" {
  cluster_identifier = aws_docdb_cluster.user_db.id
  instance_class     = "db.t3.medium"
}
