package de.truzzt.clearinghouse.edc.app.delegate;

import com.fasterxml.jackson.databind.ObjectMapper;
import de.truzzt.clearinghouse.edc.app.message.QueryMessageRequest;
import de.truzzt.clearinghouse.edc.app.message.QueryMessageResponse;
import de.truzzt.clearinghouse.edc.tests.TestUtils;
import de.truzzt.clearinghouse.edc.types.HandlerRequest;
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

class QueryMessageDelegateTest {

    @Mock
    private Monitor monitor;
    @Mock
    private QueryMessageDelegate senderDelegate;

    private final ObjectMapper mapper = new ObjectMapper();

    @BeforeEach
    public void setUp() {
        MockitoAnnotations.openMocks(this);
        senderDelegate = spy(new QueryMessageDelegate(monitor, mapper));
    }

    @Test
    public void successfulBuildCompleteRequestUrl() {

        HandlerRequest request = TestUtils.getValidHandlerQueryMessageRequest(mapper);

        String response = senderDelegate.buildRequestUrl(TestUtils.TEST_BASE_URL, request);

        assertNotNull(response);
        assertEquals(response, "http://localhost:8000/messages/query/" +request.getPid()+
                "?page=1&size=1&sort=asc&dateFrom="+ request.getPaging().getDateFrom().toString()+
                "&dateTo="+request.getPaging().getDateFrom().toString());
    }

    @Test
    public void successfulBuildOnlySizeRequestUrl() {

        HandlerRequest request = TestUtils.getValidHandlerQueryMessageOnlySizeRequest(mapper);

        String response = senderDelegate.buildRequestUrl(TestUtils.TEST_BASE_URL, request);

        assertNotNull(response);
        assertEquals(response, "http://localhost:8000/messages/query/" +request.getPid()+
                "?size=1");
    }

    @Test
    public void successfulBuildOnlySortRequestUrl() {

        HandlerRequest request = TestUtils.getValidHandlerQueryMessageOnlySortRequest(mapper);

        String response = senderDelegate.buildRequestUrl(TestUtils.TEST_BASE_URL, request);

        assertNotNull(response);
        assertEquals(response, "http://localhost:8000/messages/query/" +request.getPid()+
                "?sort=asc");
    }

    @Test
    public void successfulBuildOnlyDateFromRequestUrl() {

        HandlerRequest request = TestUtils.getValidHandlerQueryMessageOnlyDateFromRequest(mapper);

        String response = senderDelegate.buildRequestUrl(TestUtils.TEST_BASE_URL, request);

        assertNotNull(response);
        assertEquals(response, "http://localhost:8000/messages/query/" +request.getPid()+
                "?dateFrom="+ request.getPaging().getDateFrom().toString());
    }

    @Test
    public void successfulBuildOnlyDateToRequestUrl() {

        HandlerRequest request = TestUtils.getValidHandlerQueryMessageOnlyDateToRequest(mapper);

        String response = senderDelegate.buildRequestUrl(TestUtils.TEST_BASE_URL, request);

        assertNotNull(response);
        assertEquals(response, "http://localhost:8000/messages/query/" +request.getPid()+
                "?dateTo="+ request.getPaging().getDateTo().toString());
    }

    @Test
    public void successfulBuildRequestBody() {

        HandlerRequest request = TestUtils.getValidHandlerQueryMessageRequest(mapper);

        QueryMessageRequest response = senderDelegate.buildRequestBody(request);

        assertNotNull(response);
    }

    @Test
    public void successfulBuildSuccessResponse() {

        ResponseBody body = TestUtils.getValidResponseBody();
        doReturn(TestUtils.getValidQueryMessageResponse(TestUtils.getValidQueryAppSenderRequest(mapper).getUrl(), mapper))
                .when(senderDelegate).buildSuccessResponse(any(ResponseBody.class));

        QueryMessageResponse response = senderDelegate.buildSuccessResponse(body);

        assertNotNull(response);
    }
}