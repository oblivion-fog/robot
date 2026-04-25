import rclpy
from rclpy.node import Node
from example_interfaces.msg import String


class RobotSubscriberNode(Node):
    def __init__(self):
        super().__init__("robot_sub")
        # third argument is the callback function that will be called when a message is received
        self.subscription = self.create_subscription(
            String, "topic", self.listener_callback, 10)
        self.get_logger().info("Subscriber node has been started")

    def listener_callback(self, msg):
        self.get_logger().info("Received: " + msg.data)

def main(args=None):
    rclpy.init(args=args)
    node = RobotSubscriberNode()
    rclpy.spin(node)
    rclpy.shutdown()


if __name__ == "__main__":
    main()
