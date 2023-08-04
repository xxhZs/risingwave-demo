# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
"""Client and server classes corresponding to protobuf-defined services."""
import grpc

import model_pb2 as model__pb2


class ModelStub(object):
    """get_recommendation:
    request: userid String
    response: list[itemid String] (MVP: mock response)


    interact:
    request: userid String, itemid String, action ActionType
    response: timestamp (MVP: log output --> Step I: Kafka to RW)

    """

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.GetRating = channel.unary_unary(
                '/model.Model/GetRating',
                request_serializer=model__pb2.GetRatingRequest.SerializeToString,
                response_deserializer=model__pb2.GetRatingResponse.FromString,
                )
        self.Recall = channel.unary_unary(
                '/model.Model/Recall',
                request_serializer=model__pb2.RecallRequest.SerializeToString,
                response_deserializer=model__pb2.RecallResponse.FromString,
                )


class ModelServicer(object):
    """get_recommendation:
    request: userid String
    response: list[itemid String] (MVP: mock response)


    interact:
    request: userid String, itemid String, action ActionType
    response: timestamp (MVP: log output --> Step I: Kafka to RW)

    """

    def GetRating(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def Recall(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')


def add_ModelServicer_to_server(servicer, server):
    rpc_method_handlers = {
            'GetRating': grpc.unary_unary_rpc_method_handler(
                    servicer.GetRating,
                    request_deserializer=model__pb2.GetRatingRequest.FromString,
                    response_serializer=model__pb2.GetRatingResponse.SerializeToString,
            ),
            'Recall': grpc.unary_unary_rpc_method_handler(
                    servicer.Recall,
                    request_deserializer=model__pb2.RecallRequest.FromString,
                    response_serializer=model__pb2.RecallResponse.SerializeToString,
            ),
    }
    generic_handler = grpc.method_handlers_generic_handler(
            'model.Model', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))


 # This class is part of an EXPERIMENTAL API.
class Model(object):
    """get_recommendation:
    request: userid String
    response: list[itemid String] (MVP: mock response)


    interact:
    request: userid String, itemid String, action ActionType
    response: timestamp (MVP: log output --> Step I: Kafka to RW)

    """

    @staticmethod
    def GetRating(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/model.Model/GetRating',
            model__pb2.GetRatingRequest.SerializeToString,
            model__pb2.GetRatingResponse.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def Recall(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/model.Model/Recall',
            model__pb2.RecallRequest.SerializeToString,
            model__pb2.RecallResponse.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)