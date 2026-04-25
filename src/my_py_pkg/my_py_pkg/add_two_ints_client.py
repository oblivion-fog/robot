import rclpy
from rclpy.node import Node
from example_interfaces.srv import AddTwoInts
from functools import partial


class AddtwointsClientNode(Node):
    def __init__(self):
        super().__init__("add_two_ints_client")
        self.client_ = self.create_client(AddTwoInts,'add_two_ints')

    #create a function that will call the service
    def call_add_two_ints(self,a,b):
        while not self.client_.wait_for_service(timeout_sec=1.0):
            self.get_logger().info('service not available, waiting again...')
        #create a request object and fill in the request data
        request = AddTwoInts.Request()
        request.a = a
        request.b = b
        #call the service and wait for the response
        future = self.client_.call_async(request)
        future.add_done_callback(partial(self.callback_add_two_ints,request=request))

    #create a callback function that will be called when the service call is complete
    def callback_add_two_ints(self,future,request):
        response = future.result()
        self.get_logger().info(str(request.a)+'+'+str(request.b)+'='+str(response.sum))
        

def main(args=None):
    rclpy.init(args=args)
    node = AddtwointsClientNode()
    node.call_add_two_ints(3,5)
    node.call_add_two_ints(3,10)
    rclpy.spin(node)
    rclpy.shutdown()

if __name__ == "__main__":
    main()
