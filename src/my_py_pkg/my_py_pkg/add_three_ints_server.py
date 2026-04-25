import rclpy
from rclpy.node import Node
from my_robot_interfaces.srv import AddthreeInts

class AddthreeIntsServerNode(Node):
    def __init__(self):
        super().__init__("add_three_ints_server")
        self.server_ = self.create_service(
            AddthreeInts,'add_three_ints', self.callback_add_three_ints)
        self.get_logger().info("Add three ints server has been started.")
        
    def callback_add_three_ints(self,request:AddthreeInts.Request,response:AddthreeInts.Response):
        response.sum = request.a + request.b + request.c
        self.get_logger().info(str(request.a)+'+'+str(request.b)+'+'+str(request.c)+'='+str(response.sum))
        return response

def main(args=None):
    rclpy.init(args=args)
    node = AddthreeIntsServerNode()
    rclpy.spin(node)
    rclpy.shutdown()


if __name__ == "__main__":
    main()
