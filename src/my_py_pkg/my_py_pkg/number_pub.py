import rclpy
from rclpy.node import Node
from example_interfaces.msg import Int64

class NumpubNode(Node):
    #create a publisher node that publishes a string message to a topic called "topic"
    def __init__(self):
        super().__init__("number_pub")
        self.declare_parameter("number", 5)
        self.declare_parameter("timer_period", 1.0)
        self.publisher = self.create_publisher(
            Int64,  "num_topic", 10)
        timer_period = self.get_parameter("timer_period").value
        self.timer = self.create_timer(timer_period, self.publish_message)
        self.get_logger().info("Publisher node has been started")
    #create a publisher that publishes a string message 
    def publish_message(self):
        msg = Int64()
        msg.data = self.get_parameter("number").value
        self.publisher.publish(msg)
        # self.get_logger().info("Publishing: " + str(msg.data))

def main(args=None):
    rclpy.init(args=args)
    node = NumpubNode()
    rclpy.spin(node)
    rclpy.shutdown()


if __name__ == "__main__":
    main()
