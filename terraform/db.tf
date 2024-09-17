resource "aws_docdb_cluster" "user_db" {
  cluster_identifier = "biglnotation-user-db"

  port = 27017

  master_username = "biglnotationDBdev"
  master_password = "WeAreTakingABigLOnThis"

  db_subnet_group_name            = aws_docdb_subnet_group.pp_service.id
  db_cluster_parameter_group_name = aws_docdb_cluster_parameter_group.user_service.name

  deletion_protection = true
}

resource "aws_docdb_subnet_group" "pp_service" {
  name       = "biglnotation_pp_service_user_db_subnet"
  subnet_ids = [aws_subnet.sn1_private.id, aws_subnet.sn2_private.id]
}

resource "aws_docdb_cluster_instance" "user_db" {
  cluster_identifier = aws_docdb_cluster.user_db.id
  instance_class     = "db.t3.medium"
}

resource "aws_docdb_cluster_parameter_group" "user_service" {
  family = "docdb5.0"
  name   = "user-service-params"

  parameter {
    name  = "tls"
    value = "disabled"
  }
}