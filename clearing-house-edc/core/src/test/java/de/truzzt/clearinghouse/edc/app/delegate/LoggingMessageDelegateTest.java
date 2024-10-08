package de.truzzt.clearinghouse.edc.app.delegate;

import com.fasterxml.jackson.databind.ObjectMapper;
import de.truzzt.clearinghouse.edc.types.HandlerRequest;
import de.truzzt.clearinghouse.edc.app.message.LoggingMessageRequest;
import de.truzzt.clearinghouse.edc.app.message.LoggingMessageResponse;
import de.truzzt.clearinghouse.edc.tests.TestUtils;
import okhttp3.ResponseBody;
import org.eclipse.edc.spi.monitor.Monitor;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.Mock;
import org.mockito.MockitoAnnotations;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.Mockito.doReturn;
import static org.mockito.Mockito.spy;

class LoggingMessageDelegateTest {

    @Mock
    private Monitor monitor;
    @Mock
    private LoggingMessageDelegate senderDelegate;

    private final ObjectMapper mapper = new ObjectMapper();

    @BeforeEach
    public void setUp() {
        MockitoAnnotations.openMocks(this);
        senderDelegate = spy(new LoggingMessageDelegate(monitor, mapper));
    }

    @Test
    public void successfulBuildRequestUrl() {

        HandlerRequest request = TestUtils.getValidHandlerRequest(mapper);

        String response = senderDelegate.buildRequestUrl(TestUtils.TEST_BASE_URL, request);

        assertNotNull(response);
        assertEquals(response, "http://localhost:8000/messages/log/" +request.getPid());
    }

    @Test
    public void successfulBuildRequestBody() {

        HandlerRequest request = TestUtils.getValidHandlerRequest(mapper);

        LoggingMessageRequest response = senderDelegate.buildRequestBody(request);

        assertNotNull(response);
    }

    @Test
    public void successfulBuildSuccessResponse() {

        ResponseBody body = TestUtils.getValidResponseBody();
        doReturn(TestUtils.getValidLoggingMessageResponse())
                .when(senderDelegate).buildSuccessResponse(any(ResponseBody.class));

        LoggingMessageResponse response = senderDelegate.buildSuccessResponse(body);

        assertNotNull(response);
    }
}