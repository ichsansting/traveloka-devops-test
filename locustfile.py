from locust import HttpUser, TaskSet, task, between

class UserTasks(TaskSet):
    
    @task
    def root_page(self):
        self.client.get("/")

class WebsiteUser(HttpUser):
    host = "http://localhost"
    # wait_time = between(1, 1)
    tasks = [UserTasks]