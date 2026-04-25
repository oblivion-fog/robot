import rclpy
from rclpy.node import Node
from my_robot_interfaces.msg import Weather

class WeathershowNode(Node):
    def __init__(self):
        super().__init__("weather_show_pub")
        self.weather_pub_= self.create_publisher(Weather, "weather", 10)
        self.timer_= self.create_timer(1.0, self.publish_weather)
        self.get_logger().info("Weather Show Publisher Node has been started.")
    def publish_weather(self):
        weather_msg = Weather()
        weather_msg.temperature = 25.0
        weather_msg.humidity = 60.0
        self.weather_pub_.publish(weather_msg)
        self.get_logger().info(f"Published Weather: Temperature={weather_msg.temperature}°C, Humidity={weather_msg.humidity}%")

def main(args=None):
    rclpy.init(args=args)
    node = WeathershowNode()
    rclpy.spin(node)
    rclpy.shutdown()


if __name__ == "__main__":
    main()
