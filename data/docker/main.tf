terraform {
  required_providers {
    docker = {
      source  = "kreuzwerker/docker"
      version = ">= 2.13.0"
    }
  }
}

provider "docker" {}

resource "docker_image" "ubuntu" {
  name = "ubuntu:latest"
  keep_locally = false
}

resource "docker_container" "ubuntu" {
  image = docker_image.ubuntu.image_id
  name  = "ubuntu"
  publish_all_ports = true
  command = ["tail", "-f", "/dev/null"]
}

resource "docker_container" "python" {
  image = "python:3.8"
  name  = "python"
  publish_all_ports = true
  command = ["tail", "-f", "/dev/null"]
}

resource "docker_container" "azure" {
  image = "mcr.microsoft.com/azure-cli"
  name  = "azure"
  publish_all_ports = true
  command = ["tail", "-f", "/dev/null"]
}
