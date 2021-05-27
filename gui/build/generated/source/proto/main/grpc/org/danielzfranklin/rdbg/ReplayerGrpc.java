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
    comments = "Source: replayer.proto")
public final class ReplayerGrpc {

  private ReplayerGrpc() {}

  public static final String SERVICE_NAME = "org.danielzfranklin.rdbg.Replayer";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<org.danielzfranklin.rdbg.ReplayerOuterClass.CreateRequest,
      org.danielzfranklin.rdbg.ReplayerOuterClass.CreateReply> getCreateMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "create",
      requestType = org.danielzfranklin.rdbg.ReplayerOuterClass.CreateRequest.class,
      responseType = org.danielzfranklin.rdbg.ReplayerOuterClass.CreateReply.class,
      methodType = io.grpc.MethodDescriptor.MethodType.SERVER_STREAMING)
  public static io.grpc.MethodDescriptor<org.danielzfranklin.rdbg.ReplayerOuterClass.CreateRequest,
      org.danielzfranklin.rdbg.ReplayerOuterClass.CreateReply> getCreateMethod() {
    io.grpc.MethodDescriptor<org.danielzfranklin.rdbg.ReplayerOuterClass.CreateRequest, org.danielzfranklin.rdbg.ReplayerOuterClass.CreateReply> getCreateMethod;
    if ((getCreateMethod = ReplayerGrpc.getCreateMethod) == null) {
      synchronized (ReplayerGrpc.class) {
        if ((getCreateMethod = ReplayerGrpc.getCreateMethod) == null) {
          ReplayerGrpc.getCreateMethod = getCreateMethod = 
              io.grpc.MethodDescriptor.<org.danielzfranklin.rdbg.ReplayerOuterClass.CreateRequest, org.danielzfranklin.rdbg.ReplayerOuterClass.CreateReply>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.SERVER_STREAMING)
              .setFullMethodName(generateFullMethodName(
                  "org.danielzfranklin.rdbg.Replayer", "create"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  org.danielzfranklin.rdbg.ReplayerOuterClass.CreateRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  org.danielzfranklin.rdbg.ReplayerOuterClass.CreateReply.getDefaultInstance()))
                  .setSchemaDescriptor(new ReplayerMethodDescriptorSupplier("create"))
                  .build();
          }
        }
     }
     return getCreateMethod;
  }

  private static volatile io.grpc.MethodDescriptor<org.danielzfranklin.rdbg.ReplayerOuterClass.IndexRequest,
      org.danielzfranklin.rdbg.ReplayerOuterClass.IndexReply> getIndexMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "index",
      requestType = org.danielzfranklin.rdbg.ReplayerOuterClass.IndexRequest.class,
      responseType = org.danielzfranklin.rdbg.ReplayerOuterClass.IndexReply.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<org.danielzfranklin.rdbg.ReplayerOuterClass.IndexRequest,
      org.danielzfranklin.rdbg.ReplayerOuterClass.IndexReply> getIndexMethod() {
    io.grpc.MethodDescriptor<org.danielzfranklin.rdbg.ReplayerOuterClass.IndexRequest, org.danielzfranklin.rdbg.ReplayerOuterClass.IndexReply> getIndexMethod;
    if ((getIndexMethod = ReplayerGrpc.getIndexMethod) == null) {
      synchronized (ReplayerGrpc.class) {
        if ((getIndexMethod = ReplayerGrpc.getIndexMethod) == null) {
          ReplayerGrpc.getIndexMethod = getIndexMethod = 
              io.grpc.MethodDescriptor.<org.danielzfranklin.rdbg.ReplayerOuterClass.IndexRequest, org.danielzfranklin.rdbg.ReplayerOuterClass.IndexReply>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(
                  "org.danielzfranklin.rdbg.Replayer", "index"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  org.danielzfranklin.rdbg.ReplayerOuterClass.IndexRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  org.danielzfranklin.rdbg.ReplayerOuterClass.IndexReply.getDefaultInstance()))
                  .setSchemaDescriptor(new ReplayerMethodDescriptorSupplier("index"))
                  .build();
          }
        }
     }
     return getIndexMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static ReplayerStub newStub(io.grpc.Channel channel) {
    return new ReplayerStub(channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static ReplayerBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    return new ReplayerBlockingStub(channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static ReplayerFutureStub newFutureStub(
      io.grpc.Channel channel) {
    return new ReplayerFutureStub(channel);
  }

  /**
   */
  public static abstract class ReplayerImplBase implements io.grpc.BindableService {

    /**
     * <pre>
     *&#47; Only one CreateReply will be returned. Your session token is only valid
     * / for as long as you keep listening to the stream.
     * </pre>
     */
    public void create(org.danielzfranklin.rdbg.ReplayerOuterClass.CreateRequest request,
        io.grpc.stub.StreamObserver<org.danielzfranklin.rdbg.ReplayerOuterClass.CreateReply> responseObserver) {
      asyncUnimplementedUnaryCall(getCreateMethod(), responseObserver);
    }

    /**
     */
    public void index(org.danielzfranklin.rdbg.ReplayerOuterClass.IndexRequest request,
        io.grpc.stub.StreamObserver<org.danielzfranklin.rdbg.ReplayerOuterClass.IndexReply> responseObserver) {
      asyncUnimplementedUnaryCall(getIndexMethod(), responseObserver);
    }

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return io.grpc.ServerServiceDefinition.builder(getServiceDescriptor())
          .addMethod(
            getCreateMethod(),
            asyncServerStreamingCall(
              new MethodHandlers<
                org.danielzfranklin.rdbg.ReplayerOuterClass.CreateRequest,
                org.danielzfranklin.rdbg.ReplayerOuterClass.CreateReply>(
                  this, METHODID_CREATE)))
          .addMethod(
            getIndexMethod(),
            asyncUnaryCall(
              new MethodHandlers<
                org.danielzfranklin.rdbg.ReplayerOuterClass.IndexRequest,
                org.danielzfranklin.rdbg.ReplayerOuterClass.IndexReply>(
                  this, METHODID_INDEX)))
          .build();
    }
  }

  /**
   */
  public static final class ReplayerStub extends io.grpc.stub.AbstractStub<ReplayerStub> {
    private ReplayerStub(io.grpc.Channel channel) {
      super(channel);
    }

    private ReplayerStub(io.grpc.Channel channel,
        io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected ReplayerStub build(io.grpc.Channel channel,
        io.grpc.CallOptions callOptions) {
      return new ReplayerStub(channel, callOptions);
    }

    /**
     * <pre>
     *&#47; Only one CreateReply will be returned. Your session token is only valid
     * / for as long as you keep listening to the stream.
     * </pre>
     */
    public void create(org.danielzfranklin.rdbg.ReplayerOuterClass.CreateRequest request,
        io.grpc.stub.StreamObserver<org.danielzfranklin.rdbg.ReplayerOuterClass.CreateReply> responseObserver) {
      asyncServerStreamingCall(
          getChannel().newCall(getCreateMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void index(org.danielzfranklin.rdbg.ReplayerOuterClass.IndexRequest request,
        io.grpc.stub.StreamObserver<org.danielzfranklin.rdbg.ReplayerOuterClass.IndexReply> responseObserver) {
      asyncUnaryCall(
          getChannel().newCall(getIndexMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   */
  public static final class ReplayerBlockingStub extends io.grpc.stub.AbstractStub<ReplayerBlockingStub> {
    private ReplayerBlockingStub(io.grpc.Channel channel) {
      super(channel);
    }

    private ReplayerBlockingStub(io.grpc.Channel channel,
        io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected ReplayerBlockingStub build(io.grpc.Channel channel,
        io.grpc.CallOptions callOptions) {
      return new ReplayerBlockingStub(channel, callOptions);
    }

    /**
     * <pre>
     *&#47; Only one CreateReply will be returned. Your session token is only valid
     * / for as long as you keep listening to the stream.
     * </pre>
     */
    public java.util.Iterator<org.danielzfranklin.rdbg.ReplayerOuterClass.CreateReply> create(
        org.danielzfranklin.rdbg.ReplayerOuterClass.CreateRequest request) {
      return blockingServerStreamingCall(
          getChannel(), getCreateMethod(), getCallOptions(), request);
    }

    /**
     */
    public org.danielzfranklin.rdbg.ReplayerOuterClass.IndexReply index(org.danielzfranklin.rdbg.ReplayerOuterClass.IndexRequest request) {
      return blockingUnaryCall(
          getChannel(), getIndexMethod(), getCallOptions(), request);
    }
  }

  /**
   */
  public static final class ReplayerFutureStub extends io.grpc.stub.AbstractStub<ReplayerFutureStub> {
    private ReplayerFutureStub(io.grpc.Channel channel) {
      super(channel);
    }

    private ReplayerFutureStub(io.grpc.Channel channel,
        io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected ReplayerFutureStub build(io.grpc.Channel channel,
        io.grpc.CallOptions callOptions) {
      return new ReplayerFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<org.danielzfranklin.rdbg.ReplayerOuterClass.IndexReply> index(
        org.danielzfranklin.rdbg.ReplayerOuterClass.IndexRequest request) {
      return futureUnaryCall(
          getChannel().newCall(getIndexMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_CREATE = 0;
  private static final int METHODID_INDEX = 1;

  private static final class MethodHandlers<Req, Resp> implements
      io.grpc.stub.ServerCalls.UnaryMethod<Req, Resp>,
      io.grpc.stub.ServerCalls.ServerStreamingMethod<Req, Resp>,
      io.grpc.stub.ServerCalls.ClientStreamingMethod<Req, Resp>,
      io.grpc.stub.ServerCalls.BidiStreamingMethod<Req, Resp> {
    private final ReplayerImplBase serviceImpl;
    private final int methodId;

    MethodHandlers(ReplayerImplBase serviceImpl, int methodId) {
      this.serviceImpl = serviceImpl;
      this.methodId = methodId;
    }

    @java.lang.Override
    @java.lang.SuppressWarnings("unchecked")
    public void invoke(Req request, io.grpc.stub.StreamObserver<Resp> responseObserver) {
      switch (methodId) {
        case METHODID_CREATE:
          serviceImpl.create((org.danielzfranklin.rdbg.ReplayerOuterClass.CreateRequest) request,
              (io.grpc.stub.StreamObserver<org.danielzfranklin.rdbg.ReplayerOuterClass.CreateReply>) responseObserver);
          break;
        case METHODID_INDEX:
          serviceImpl.index((org.danielzfranklin.rdbg.ReplayerOuterClass.IndexRequest) request,
              (io.grpc.stub.StreamObserver<org.danielzfranklin.rdbg.ReplayerOuterClass.IndexReply>) responseObserver);
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

  private static abstract class ReplayerBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    ReplayerBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return org.danielzfranklin.rdbg.ReplayerOuterClass.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Replayer");
    }
  }

  private static final class ReplayerFileDescriptorSupplier
      extends ReplayerBaseDescriptorSupplier {
    ReplayerFileDescriptorSupplier() {}
  }

  private static final class ReplayerMethodDescriptorSupplier
      extends ReplayerBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final String methodName;

    ReplayerMethodDescriptorSupplier(String methodName) {
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
      synchronized (ReplayerGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new ReplayerFileDescriptorSupplier())
              .addMethod(getCreateMethod())
              .addMethod(getIndexMethod())
              .build();
        }
      }
    }
    return result;
  }
}
