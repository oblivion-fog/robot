import rclpy
from rclpy.node import Node
from turtlesim.msg import Pose
from geometry_msgs.msg import Twist
import math
from my_robot_interfaces.msg import Turtle,TurtleArray
from my_robot_interfaces.srv import CatchTurtle
from functools import partial

class TurtleControllerNode(Node):
    def __init__(self):
        super().__init__("turtle_controller")
       
        self.declare_parameter("catch_closest_turtle_first", True)
        self.catch_closest_turtle_first_=self.get_parameter("catch_closest_turtle_first").value
        
        self.turtle_to_catch_:Turtle = None
        self.pose_ :Pose = None
        self.cmd_vel_pub_ = self.create_publisher(
            Twist, "/turtle1/cmd_vel", 10)
        self.pose_sub_ = self.create_subscription(
            Pose, "/turtle1/pose", self.pose_callback, 10)
        
        self.alive_turtles_sub_ = self.create_subscription(
            TurtleArray, "alive_turtles", self.alive_turtles_callback, 10)
        self.catch_turtle_client_ = self.create_client(
            CatchTurtle, "catch_turtle")
        #创建控制循环定时器，周期为0.1秒，调用control_loop方法
        self.create_loop_timer_=self.create_timer(0.1, self.control_loop)
    def pose_callback(self, msg: Pose):
        self.pose_ = msg
        # self.get_logger().info(
        #     f"Received pose: x={msg.x}, y={msg.y}, theta={msg.theta}")
    def alive_turtles_callback(self, turtles_list: TurtleArray):
        if len(turtles_list.turtles)>0:
            if self.catch_closest_turtle_first_:
                closest_turtle=None
                min_turtle_distance=float('inf')
                # 找到最近的乌龟
                for turtle in turtles_list.turtles:
                    distance=(turtle.x-self.pose_.x)**2+(turtle.y-self.pose_.y)**2
                    if distance<min_turtle_distance or closest_turtle == None:
                        min_turtle_distance=distance
                        closest_turtle=turtle
                self.turtle_to_catch_ = closest_turtle
            else:
                self.turtle_to_catch_ = turtles_list.turtles[0]
    # 控制循环
    def control_loop(self):
        if self.pose_ is None or self.turtle_to_catch_ is None:
            # self.get_logger().info("No pose or turtle to catch received yet")
            return
        #calculate error position
        error_x = self.turtle_to_catch_.x - self.pose_.x
        error_y = self.turtle_to_catch_.y - self.pose_.y
        distance=(error_x**2+error_y**2)**0.5
        # self.get_logger().info(f"Distance to target: {distance}")
        #
        cmd = Twist()
        if distance>0.05:
            #position - 限制最大线性速度为1.0
            cmd.linear.x = 2.5*distance
            #orientation - 限制最大角速度为1.5
            goal_theta = math.atan2(error_y, error_x)
            diff=goal_theta - self.pose_.theta
            if diff > math.pi:
                diff -= 2 * math.pi
            elif diff < -math.pi:
                diff += 2 * math.pi
            cmd.angular.z = 6*diff
            # self.get_logger().info(f"Publishing velocity: linear={cmd.linear.x}, angular={cmd.angular.z}")
        else:
            cmd.linear.x = 0.0
            cmd.angular.z = 0.0
            self.call_catch_turtle_service(self.turtle_to_catch_.name)
            self.turtle_to_catch_ = None
        self.cmd_vel_pub_.publish(cmd)
    # 调用捕获服务
    def call_catch_turtle_service(self,turtle_name:str):
        if not self.catch_turtle_client_.wait_for_service(timeout_sec=1.0):
            self.get_logger().error("Catch turtle service not available")
            return
        
        request = CatchTurtle.Request()
        request.name=turtle_name
        # 发送服务请求
        future=self.catch_turtle_client_.call_async(request)
        future.add_done_callback(
            partial(self.callback_catch_service,turtle_name=turtle_name))
    # 服务回调函数
    def callback_catch_service(self, future, turtle_name:str):
        response:CatchTurtle.Response = future.result()
        if response.success:
            self.get_logger().info(f"Successfully caught turtle {turtle_name}")
        else:
            self.get_logger().info(f"Failed to catch turtle {turtle_name}")
def main(args=None):
    rclpy.init(args=args)
    node = TurtleControllerNode()
    rclpy.spin(node)
    rclpy.shutdown()


if __name__ == "__main__":
    main()
