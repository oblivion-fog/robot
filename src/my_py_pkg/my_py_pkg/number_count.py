import rclpy
from rclpy.node import Node
from example_interfaces.msg import Int64


class NumCountNode(Node):
    def __init__(self):
        super().__init__("number_count")
        self.count = 0
        self.subscription = self.create_subscription(
            Int64, "num_topic", self.listener_callback, 10)
        self.get_logger().info("Number counter has been started")
    def listener_callback(self, msg):
        self.count += msg.data
        self.get_logger().info("Current count: " + str(self.count))
def main(args=None):
    rclpy.init(args=args)
    node = NumCountNode()
    rclpy.spin(node)
    rclpy.shutdown()


if __name__ == "__main__":
    main()
