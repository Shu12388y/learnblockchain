## Kubernetes

Kubernetes is a container archestration engine which as the name
suggest lets you create, delete and update containers.


This is useful when

1. You have your docker image in the docker registry and want to deploy it in a cloud native fashion.

2. You want to not worry about patching, crashes. You want the system to
auto heal.

3. You want to autoscale with some simple constructs.

4. You want to observe your system in a simple dashboard.




## Nodes 
In kubernetes, you can create and connect various machines together, all which are running kubernetes. Every machine here is known as node


There are two types of nodes

1. Master nodes(Control Panel): The node that takes care of deploying the 
container/healing them /listening to the developer understand what to deploy

 - API Server   
 - etcd
 - kube-scheduler
 - kube-controller-manager
 - Kube-cloud-manager





- Commands

1. Create cluster
kind create cluster --name cluster-name