import rclpy
from rclpy.node import Node
from example_interfaces.msg import String

class RobotpublisherNode(Node):
    #create a publisher node that publishes a string message to a topic called "topic"
    def __init__(self):
        super().__init__("robot_pub")
        self.publisher = self.create_publisher(
            String,  "topic", 10)
        self.timer = self.create_timer(1.0, self.pubilsh_message)
        self.get_logger().info("Publisher node has been started")
    #create a publisher that publishes a string message 
    def pubilsh_message(self):
        msg = String()
        msg.data = "Hello World"
        self.publisher.publish(msg)
        self.get_logger().info("Publishing: " + msg.data)

def main(args=None):
    rclpy.init(args=args)
    node = RobotpublisherNode()
    rclpy.spin(node)
    rclpy.shutdown()


if __name__ == "__main__":
    main()
