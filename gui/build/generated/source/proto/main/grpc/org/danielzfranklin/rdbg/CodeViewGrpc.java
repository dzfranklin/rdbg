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
    comments = "Source: code_view.proto")
public final class CodeViewGrpc {

  private CodeViewGrpc() {}

  public static final String SERVICE_NAME = "org.danielzfranklin.rdbg.CodeView";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<org.danielzfranklin.rdbg.CodeViewOuterClass.SourceRequest,
      org.danielzfranklin.rdbg.CodeViewOuterClass.SourceReply> getSourceMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "source",
      requestType = org.danielzfranklin.rdbg.CodeViewOuterClass.SourceRequest.class,
      responseType = org.danielzfranklin.rdbg.CodeViewOuterClass.SourceReply.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<org.danielzfranklin.rdbg.CodeViewOuterClass.SourceRequest,
      org.danielzfranklin.rdbg.CodeViewOuterClass.SourceReply> getSourceMethod() {
    io.grpc.MethodDescriptor<org.danielzfranklin.rdbg.CodeViewOuterClass.SourceRequest, org.danielzfranklin.rdbg.CodeViewOuterClass.SourceReply> getSourceMethod;
    if ((getSourceMethod = CodeViewGrpc.getSourceMethod) == null) {
      synchronized (CodeViewGrpc.class) {
        if ((getSourceMethod = CodeViewGrpc.getSourceMethod) == null) {
          CodeViewGrpc.getSourceMethod = getSourceMethod = 
              io.grpc.MethodDescriptor.<org.danielzfranklin.rdbg.CodeViewOuterClass.SourceRequest, org.danielzfranklin.rdbg.CodeViewOuterClass.SourceReply>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(
                  "org.danielzfranklin.rdbg.CodeView", "source"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  org.danielzfranklin.rdbg.CodeViewOuterClass.SourceRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  org.danielzfranklin.rdbg.CodeViewOuterClass.SourceReply.getDefaultInstance()))
                  .setSchemaDescriptor(new CodeViewMethodDescriptorSupplier("source"))
                  .build();
          }
        }
     }
     return getSourceMethod;
  }

  private static volatile io.grpc.MethodDescriptor<org.danielzfranklin.rdbg.CodeViewOuterClass.HighlightRequest,
      org.danielzfranklin.rdbg.CodeViewOuterClass.HighlightReply> getHighlightMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "highlight",
      requestType = org.danielzfranklin.rdbg.CodeViewOuterClass.HighlightRequest.class,
      responseType = org.danielzfranklin.rdbg.CodeViewOuterClass.HighlightReply.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<org.danielzfranklin.rdbg.CodeViewOuterClass.HighlightRequest,
      org.danielzfranklin.rdbg.CodeViewOuterClass.HighlightReply> getHighlightMethod() {
    io.grpc.MethodDescriptor<org.danielzfranklin.rdbg.CodeViewOuterClass.HighlightRequest, org.danielzfranklin.rdbg.CodeViewOuterClass.HighlightReply> getHighlightMethod;
    if ((getHighlightMethod = CodeViewGrpc.getHighlightMethod) == null) {
      synchronized (CodeViewGrpc.class) {
        if ((getHighlightMethod = CodeViewGrpc.getHighlightMethod) == null) {
          CodeViewGrpc.getHighlightMethod = getHighlightMethod = 
              io.grpc.MethodDescriptor.<org.danielzfranklin.rdbg.CodeViewOuterClass.HighlightRequest, org.danielzfranklin.rdbg.CodeViewOuterClass.HighlightReply>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(
                  "org.danielzfranklin.rdbg.CodeView", "highlight"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  org.danielzfranklin.rdbg.CodeViewOuterClass.HighlightRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  org.danielzfranklin.rdbg.CodeViewOuterClass.HighlightReply.getDefaultInstance()))
                  .setSchemaDescriptor(new CodeViewMethodDescriptorSupplier("highlight"))
                  .build();
          }
        }
     }
     return getHighlightMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static CodeViewStub newStub(io.grpc.Channel channel) {
    return new CodeViewStub(channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static CodeViewBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    return new CodeViewBlockingStub(channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static CodeViewFutureStub newFutureStub(
      io.grpc.Channel channel) {
    return new CodeViewFutureStub(channel);
  }

  /**
   */
  public static abstract class CodeViewImplBase implements io.grpc.BindableService {

    /**
     * <pre>
     * Separated so we can display source while waiting for highlights
     * </pre>
     */
    public void source(org.danielzfranklin.rdbg.CodeViewOuterClass.SourceRequest request,
        io.grpc.stub.StreamObserver<org.danielzfranklin.rdbg.CodeViewOuterClass.SourceReply> responseObserver) {
      asyncUnimplementedUnaryCall(getSourceMethod(), responseObserver);
    }

    /**
     */
    public void highlight(org.danielzfranklin.rdbg.CodeViewOuterClass.HighlightRequest request,
        io.grpc.stub.StreamObserver<org.danielzfranklin.rdbg.CodeViewOuterClass.HighlightReply> responseObserver) {
      asyncUnimplementedUnaryCall(getHighlightMethod(), responseObserver);
    }

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return io.grpc.ServerServiceDefinition.builder(getServiceDescriptor())
          .addMethod(
            getSourceMethod(),
            asyncUnaryCall(
              new MethodHandlers<
                org.danielzfranklin.rdbg.CodeViewOuterClass.SourceRequest,
                org.danielzfranklin.rdbg.CodeViewOuterClass.SourceReply>(
                  this, METHODID_SOURCE)))
          .addMethod(
            getHighlightMethod(),
            asyncUnaryCall(
              new MethodHandlers<
                org.danielzfranklin.rdbg.CodeViewOuterClass.HighlightRequest,
                org.danielzfranklin.rdbg.CodeViewOuterClass.HighlightReply>(
                  this, METHODID_HIGHLIGHT)))
          .build();
    }
  }

  /**
   */
  public static final class CodeViewStub extends io.grpc.stub.AbstractStub<CodeViewStub> {
    private CodeViewStub(io.grpc.Channel channel) {
      super(channel);
    }

    private CodeViewStub(io.grpc.Channel channel,
        io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CodeViewStub build(io.grpc.Channel channel,
        io.grpc.CallOptions callOptions) {
      return new CodeViewStub(channel, callOptions);
    }

    /**
     * <pre>
     * Separated so we can display source while waiting for highlights
     * </pre>
     */
    public void source(org.danielzfranklin.rdbg.CodeViewOuterClass.SourceRequest request,
        io.grpc.stub.StreamObserver<org.danielzfranklin.rdbg.CodeViewOuterClass.SourceReply> responseObserver) {
      asyncUnaryCall(
          getChannel().newCall(getSourceMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void highlight(org.danielzfranklin.rdbg.CodeViewOuterClass.HighlightRequest request,
        io.grpc.stub.StreamObserver<org.danielzfranklin.rdbg.CodeViewOuterClass.HighlightReply> responseObserver) {
      asyncUnaryCall(
          getChannel().newCall(getHighlightMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   */
  public static final class CodeViewBlockingStub extends io.grpc.stub.AbstractStub<CodeViewBlockingStub> {
    private CodeViewBlockingStub(io.grpc.Channel channel) {
      super(channel);
    }

    private CodeViewBlockingStub(io.grpc.Channel channel,
        io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CodeViewBlockingStub build(io.grpc.Channel channel,
        io.grpc.CallOptions callOptions) {
      return new CodeViewBlockingStub(channel, callOptions);
    }

    /**
     * <pre>
     * Separated so we can display source while waiting for highlights
     * </pre>
     */
    public org.danielzfranklin.rdbg.CodeViewOuterClass.SourceReply source(org.danielzfranklin.rdbg.CodeViewOuterClass.SourceRequest request) {
      return blockingUnaryCall(
          getChannel(), getSourceMethod(), getCallOptions(), request);
    }

    /**
     */
    public org.danielzfranklin.rdbg.CodeViewOuterClass.HighlightReply highlight(org.danielzfranklin.rdbg.CodeViewOuterClass.HighlightRequest request) {
      return blockingUnaryCall(
          getChannel(), getHighlightMethod(), getCallOptions(), request);
    }
  }

  /**
   */
  public static final class CodeViewFutureStub extends io.grpc.stub.AbstractStub<CodeViewFutureStub> {
    private CodeViewFutureStub(io.grpc.Channel channel) {
      super(channel);
    }

    private CodeViewFutureStub(io.grpc.Channel channel,
        io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CodeViewFutureStub build(io.grpc.Channel channel,
        io.grpc.CallOptions callOptions) {
      return new CodeViewFutureStub(channel, callOptions);
    }

    /**
     * <pre>
     * Separated so we can display source while waiting for highlights
     * </pre>
     */
    public com.google.common.util.concurrent.ListenableFuture<org.danielzfranklin.rdbg.CodeViewOuterClass.SourceReply> source(
        org.danielzfranklin.rdbg.CodeViewOuterClass.SourceRequest request) {
      return futureUnaryCall(
          getChannel().newCall(getSourceMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<org.danielzfranklin.rdbg.CodeViewOuterClass.HighlightReply> highlight(
        org.danielzfranklin.rdbg.CodeViewOuterClass.HighlightRequest request) {
      return futureUnaryCall(
          getChannel().newCall(getHighlightMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_SOURCE = 0;
  private static final int METHODID_HIGHLIGHT = 1;

  private static final class MethodHandlers<Req, Resp> implements
      io.grpc.stub.ServerCalls.UnaryMethod<Req, Resp>,
      io.grpc.stub.ServerCalls.ServerStreamingMethod<Req, Resp>,
      io.grpc.stub.ServerCalls.ClientStreamingMethod<Req, Resp>,
      io.grpc.stub.ServerCalls.BidiStreamingMethod<Req, Resp> {
    private final CodeViewImplBase serviceImpl;
    private final int methodId;

    MethodHandlers(CodeViewImplBase serviceImpl, int methodId) {
      this.serviceImpl = serviceImpl;
      this.methodId = methodId;
    }

    @java.lang.Override
    @java.lang.SuppressWarnings("unchecked")
    public void invoke(Req request, io.grpc.stub.StreamObserver<Resp> responseObserver) {
      switch (methodId) {
        case METHODID_SOURCE:
          serviceImpl.source((org.danielzfranklin.rdbg.CodeViewOuterClass.SourceRequest) request,
              (io.grpc.stub.StreamObserver<org.danielzfranklin.rdbg.CodeViewOuterClass.SourceReply>) responseObserver);
          break;
        case METHODID_HIGHLIGHT:
          serviceImpl.highlight((org.danielzfranklin.rdbg.CodeViewOuterClass.HighlightRequest) request,
              (io.grpc.stub.StreamObserver<org.danielzfranklin.rdbg.CodeViewOuterClass.HighlightReply>) responseObserver);
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

  private static abstract class CodeViewBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    CodeViewBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return org.danielzfranklin.rdbg.CodeViewOuterClass.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("CodeView");
    }
  }

  private static final class CodeViewFileDescriptorSupplier
      extends CodeViewBaseDescriptorSupplier {
    CodeViewFileDescriptorSupplier() {}
  }

  private static final class CodeViewMethodDescriptorSupplier
      extends CodeViewBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final String methodName;

    CodeViewMethodDescriptorSupplier(String methodName) {
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
      synchronized (CodeViewGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new CodeViewFileDescriptorSupplier())
              .addMethod(getSourceMethod())
              .addMethod(getHighlightMethod())
              .build();
        }
      }
    }
    return result;
  }
}
