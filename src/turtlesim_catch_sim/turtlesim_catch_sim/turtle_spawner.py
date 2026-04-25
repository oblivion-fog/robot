#spawns turtles in turtlesim ,kill turtles and manages the alive turtles list

import rclpy
from rclpy.node import Node
from functools import partial
import random
import math
from turtlesim.srv import Spawn
from my_robot_interfaces.msg import Turtle
from my_robot_interfaces.msg import TurtleArray
from my_robot_interfaces.srv import CatchTurtle
from turtlesim.srv import Kill

class TurtleSpawnNode(Node):
    def __init__(self):
        super().__init__("turtle_spawner")
        
        self.declare_parameter("turtle_name_prefix", "turtle")
        self.turtle_name_prefix_=self.get_parameter("turtle_name_prefix").value
        self.declare_parameter("turtle_frequency", 1.0)
        self.turtle_frequency_=self.get_parameter("turtle_frequency").value
        
        self.turtle_counter_=1
        self.alive_turtles_=[]
        self.alive_turtles_pub_=self.create_publisher(
            TurtleArray, "alive_turtles", 10)
        self.spawn_client_ = self.create_client(Spawn, "/spawn")
        self.kill_client_ = self.create_client(Kill, "/kill")
        self.catch_turtle_service_=self.create_service(CatchTurtle, "catch_turtle", self.callback_catch_turtle)
        self.spawn_turtle_timer_=self.create_timer(self.turtle_frequency_, self.spawn_new_turtle)
    # 处理catch_turtle服务请求
    def callback_catch_turtle(self, request: CatchTurtle.Request, response: CatchTurtle.Response):
        self.call_kill_service(request.name)
        response.success = True
        return response
    # 发布所有turtle信息
    def publish_alive_turtles(self):
        self.alive_turtles_pub_.publish(TurtleArray(turtles=self.alive_turtles_))
    #创建新turtle
    def spawn_new_turtle(self):
        self.turtle_counter_+=1
        turtle_name=self.turtle_name_prefix_+str(self.turtle_counter_)
        x=random.uniform(0.0,10.5)
        y=random.uniform(0.0,10.5)
        theta=random.uniform(0.0,2*math.pi)
        self.call_spawn_service(turtle_name,x,y,theta)
    #作用：调用spawn服务
    def call_spawn_service(self,turtle_name:str,x,y,theta):
        while not self.spawn_client_.wait_for_service():
            self.get_logger().info("Waiting for service to be available...")
            rclpy.spin_once(self, timeout_sec=1.0)
        # 构建服务请求
        request = Spawn.Request()
        request.name = turtle_name
        request.x = x
        request.y = y
        request.theta = theta
        # 发送服务请求
        future = self.spawn_client_.call_async(request)
        future.add_done_callback(
            partial(self.callback_spawn_response,request=request))
    # 处理spawn服务响应
    def callback_spawn_response(self, future, request:Spawn.Request):
        if future.done():
            response = future.result()
            if response.name != "":
                self.get_logger().info(f"Spawned turtle {response.name} successfully")
                new_turtle=Turtle()
                new_turtle.name=response.name
                new_turtle.x=request.x
                new_turtle.y=request.y
                new_turtle.theta=request.theta
                self.alive_turtles_.append(new_turtle)
                self.publish_alive_turtles()
            else:
                self.get_logger().error(f"Failed to spawn turtle {response.name}")
        else:
            self.get_logger().error(f"Failed to spawn turtle {response.name}")
    # 调用kill服务，删除turtle
    def call_kill_service(self, turtle_name:str):
        while not self.kill_client_.wait_for_service():
            self.get_logger().info("Waiting for service to be available...")
            rclpy.spin_once(self, timeout_sec=1.0)
        # 构建服务请求
        request=Kill.Request()
        request.name=turtle_name
        # 发送服务请求
        future=self.kill_client_.call_async(request)
        future.add_done_callback(
            partial(self.callback_kill_service,turtle_name=turtle_name))
    # 处理kill服务响应
    def callback_kill_service(self, future, turtle_name):
        for(i,turtle) in enumerate(self.alive_turtles_):
            if turtle.name==turtle_name:
                del self.alive_turtles_[i] 
                self.publish_alive_turtles()
                break
        
def main(args=None):
    rclpy.init(args=args)
    node = TurtleSpawnNode()
    # node.call_spawn_service("turtle2",8.0,4.0,0.0)
    rclpy.spin(node)
    rclpy.shutdown()


if __name__ == "__main__":
    main()
