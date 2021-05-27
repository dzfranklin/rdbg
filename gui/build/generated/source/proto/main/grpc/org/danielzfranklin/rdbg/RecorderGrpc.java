package org.danielzfranklin.rdbg;

import static io.grpc.MethodDescriptor.generateFullMethodName;
import static io.grpc.stub.ClientCalls.asyncBidiStreamingCall;
import static io.grpc.stub.ClientCalls.asyncClientStreamingCall;
import static io.grpc.stub.ClientCalls.asyncServerStreamingCall;
import static io.grpc.stub.ClientCalls.asyncUnaryCall;
import static io.grpc.stub.ClientCalls.blockingServerStreamingCall;
import static io.grpc.stub.ClientCalls.blockingUnaryCall;
import static io.grpc.stub.ClientCalls.futureUnaryCall;
import static io.grpc.stub.ServerCalls.asyncBidiStreamingCall;
import static io.grpc.stub.ServerCalls.asyncClientStreamingCall;
import static io.grpc.stub.ServerCalls.asyncServerStreamingCall;
import static io.grpc.stub.ServerCalls.asyncUnaryCall;
import static io.grpc.stub.ServerCalls.asyncUnimplementedStreamingCall;
import static io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall;

/**
 */
@javax.annotation.Generated(
    value = "by gRPC proto compiler (version 1.15.1)",
    comments = "Source: recorder.proto")
public final class RecorderGrpc {

  private RecorderGrpc() {}

  public static final String SERVICE_NAME = "org.danielzfranklin.rdbg.Recorder";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<org.danielzfranklin.rdbg.RecorderOuterClass.RecordTestRequest,
      org.danielzfranklin.rdbg.RecorderOuterClass.RecordTestReply> getRecordTestMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "record_test",
      requestType = org.danielzfranklin.rdbg.RecorderOuterClass.RecordTestRequest.class,
      responseType = org.danielzfranklin.rdbg.RecorderOuterClass.RecordTestReply.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<org.danielzfranklin.rdbg.RecorderOuterClass.RecordTestRequest,
      org.danielzfranklin.rdbg.RecorderOuterClass.RecordTestReply> getRecordTestMethod() {
    io.grpc.MethodDescriptor<org.danielzfranklin.rdbg.RecorderOuterClass.RecordTestRequest, org.danielzfranklin.rdbg.RecorderOuterClass.RecordTestReply> getRecordTestMethod;
    if ((getRecordTestMethod = RecorderGrpc.getRecordTestMethod) == null) {
      synchronized (RecorderGrpc.class) {
        if ((getRecordTestMethod = RecorderGrpc.getRecordTestMethod) == null) {
          RecorderGrpc.getRecordTestMethod = getRecordTestMethod = 
              io.grpc.MethodDescriptor.<org.danielzfranklin.rdbg.RecorderOuterClass.RecordTestRequest, org.danielzfranklin.rdbg.RecorderOuterClass.RecordTestReply>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(
                  "org.danielzfranklin.rdbg.Recorder", "record_test"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  org.danielzfranklin.rdbg.RecorderOuterClass.RecordTestRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  org.danielzfranklin.rdbg.RecorderOuterClass.RecordTestReply.getDefaultInstance()))
                  .setSchemaDescriptor(new RecorderMethodDescriptorSupplier("record_test"))
                  .build();
          }
        }
     }
     return getRecordTestMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static RecorderStub newStub(io.grpc.Channel channel) {
    return new RecorderStub(channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static RecorderBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    return new RecorderBlockingStub(channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static RecorderFutureStub newFutureStub(
      io.grpc.Channel channel) {
    return new RecorderFutureStub(channel);
  }

  /**
   */
  public static abstract class RecorderImplBase implements io.grpc.BindableService {

    /**
     * <pre>
     * TODO: rpc record_bin
     * TODO: rpc record_example
     * </pre>
     */
    public void recordTest(org.danielzfranklin.rdbg.RecorderOuterClass.RecordTestRequest request,
        io.grpc.stub.StreamObserver<org.danielzfranklin.rdbg.RecorderOuterClass.RecordTestReply> responseObserver) {
      asyncUnimplementedUnaryCall(getRecordTestMethod(), responseObserver);
    }

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return io.grpc.ServerServiceDefinition.builder(getServiceDescriptor())
          .addMethod(
            getRecordTestMethod(),
            asyncUnaryCall(
              new MethodHandlers<
                org.danielzfranklin.rdbg.RecorderOuterClass.RecordTestRequest,
                org.danielzfranklin.rdbg.RecorderOuterClass.RecordTestReply>(
                  this, METHODID_RECORD_TEST)))
          .build();
    }
  }

  /**
   */
  public static final class RecorderStub extends io.grpc.stub.AbstractStub<RecorderStub> {
    private RecorderStub(io.grpc.Channel channel) {
      super(channel);
    }

    private RecorderStub(io.grpc.Channel channel,
        io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected RecorderStub build(io.grpc.Channel channel,
        io.grpc.CallOptions callOptions) {
      return new RecorderStub(channel, callOptions);
    }

    /**
     * <pre>
     * TODO: rpc record_bin
     * TODO: rpc record_example
     * </pre>
     */
    public void recordTest(org.danielzfranklin.rdbg.RecorderOuterClass.RecordTestRequest request,
        io.grpc.stub.StreamObserver<org.danielzfranklin.rdbg.RecorderOuterClass.RecordTestReply> responseObserver) {
      asyncUnaryCall(
          getChannel().newCall(getRecordTestMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   */
  public static final class RecorderBlockingStub extends io.grpc.stub.AbstractStub<RecorderBlockingStub> {
    private RecorderBlockingStub(io.grpc.Channel channel) {
      super(channel);
    }

    private RecorderBlockingStub(io.grpc.Channel channel,
        io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected RecorderBlockingStub build(io.grpc.Channel channel,
        io.grpc.CallOptions callOptions) {
      return new RecorderBlockingStub(channel, callOptions);
    }

    /**
     * <pre>
     * TODO: rpc record_bin
     * TODO: rpc record_example
     * </pre>
     */
    public org.danielzfranklin.rdbg.RecorderOuterClass.RecordTestReply recordTest(org.danielzfranklin.rdbg.RecorderOuterClass.RecordTestRequest request) {
      return blockingUnaryCall(
          getChannel(), getRecordTestMethod(), getCallOptions(), request);
    }
  }

  /**
   */
  public static final class RecorderFutureStub extends io.grpc.stub.AbstractStub<RecorderFutureStub> {
    private RecorderFutureStub(io.grpc.Channel channel) {
      super(channel);
    }

    private RecorderFutureStub(io.grpc.Channel channel,
        io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected RecorderFutureStub build(io.grpc.Channel channel,
        io.grpc.CallOptions callOptions) {
      return new RecorderFutureStub(channel, callOptions);
    }

    /**
     * <pre>
     * TODO: rpc record_bin
     * TODO: rpc record_example
     * </pre>
     */
    public com.google.common.util.concurrent.ListenableFuture<org.danielzfranklin.rdbg.RecorderOuterClass.RecordTestReply> recordTest(
        org.danielzfranklin.rdbg.RecorderOuterClass.RecordTestRequest request) {
      return futureUnaryCall(
          getChannel().newCall(getRecordTestMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_RECORD_TEST = 0;

  private static final class MethodHandlers<Req, Resp> implements
      io.grpc.stub.ServerCalls.UnaryMethod<Req, Resp>,
      io.grpc.stub.ServerCalls.ServerStreamingMethod<Req, Resp>,
      io.grpc.stub.ServerCalls.ClientStreamingMethod<Req, Resp>,
      io.grpc.stub.ServerCalls.BidiStreamingMethod<Req, Resp> {
    private final RecorderImplBase serviceImpl;
    private final int methodId;

    MethodHandlers(RecorderImplBase serviceImpl, int methodId) {
      this.serviceImpl = serviceImpl;
      this.methodId = methodId;
    }

    @java.lang.Override
    @java.lang.SuppressWarnings("unchecked")
    public void invoke(Req request, io.grpc.stub.StreamObserver<Resp> responseObserver) {
      switch (methodId) {
        case METHODID_RECORD_TEST:
          serviceImpl.recordTest((org.danielzfranklin.rdbg.RecorderOuterClass.RecordTestRequest) request,
              (io.grpc.stub.StreamObserver<org.danielzfranklin.rdbg.RecorderOuterClass.RecordTestReply>) responseObserver);
          break;
        default:
          throw new AssertionError();
      }
    }

    @java.lang.Override
    @java.lang.SuppressWarnings("unchecked")
    public io.grpc.stub.StreamObserver<Req> invoke(
        io.grpc.stub.StreamObserver<Resp> responseObserver) {
      switch (methodId) {
        default:
          throw new AssertionError();
      }
    }
  }

  private static abstract class RecorderBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    RecorderBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return org.danielzfranklin.rdbg.RecorderOuterClass.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Recorder");
    }
  }

  private static final class RecorderFileDescriptorSupplier
      extends RecorderBaseDescriptorSupplier {
    RecorderFileDescriptorSupplier() {}
  }

  private static final class RecorderMethodDescriptorSupplier
      extends RecorderBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final String methodName;

    RecorderMethodDescriptorSupplier(String methodName) {
      this.methodName = methodName;
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.MethodDescriptor getMethodDescriptor() {
      return getServiceDescriptor().findMethodByName(methodName);
    }
  }

  private static volatile io.grpc.ServiceDescriptor serviceDescriptor;

  public static io.grpc.ServiceDescriptor getServiceDescriptor() {
    io.grpc.ServiceDescriptor result = serviceDescriptor;
    if (result == null) {
      synchronized (RecorderGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new RecorderFileDescriptorSupplier())
              .addMethod(getRecordTestMethod())
              .build();
        }
      }
    }
    return result;
  }
}
